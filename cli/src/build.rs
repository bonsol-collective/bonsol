use std::fs::{self, File};
use std::path::Path;
use std::time::Duration;

use anyhow::Result;
use cargo_metadata::{MetadataCommand, Package};
use indicatif::ProgressBar;
use risc0_build::{DockerOptionsBuilder, GuestOptionsBuilder};
use risc0_zkvm::compute_image_id;
use solana_sdk::signer::Signer;

use crate::common::*;
use crate::error::{BonsolCliError, ZkManifestError};

pub fn build(keypair: &impl Signer, zk_program_path: String) -> Result<()> {
    let bar = ProgressBar::new_spinner();
    bar.enable_steady_tick(Duration::from_millis(100));

    let image_path = Path::new(&zk_program_path);
    let (package, input_order) = parse_cargo_manifest(image_path)?;
    let build_result = build_zkprogram(image_path, &keypair, &package, input_order);
    let manifest_path = image_path.join(MANIFEST_JSON);
    match build_result {
        Err(e) => {
            bar.finish_with_message(format!(
                "Build failed for program '{}': {:?}",
                image_path.to_string_lossy(),
                e
            ));
            Ok(())
        }
        Ok(manifest) => {
            serde_json::to_writer_pretty(File::create(&manifest_path)?, &manifest)?;
            bar.finish_and_clear();
            println!("Build complete");
            Ok(())
        }
    }
}

fn parse_cargo_manifest_inputs(package: &Package) -> Result<Vec<String>> {
    const ZKPROGRAM: &str = "zkprogram";
    const INPUT_ORDER: &str = "input_order";

    let meta = if package.metadata.is_null() {
        return Err(ZkManifestError::MissingPackageMetadata(
            package.manifest_path.as_str().to_owned(),
        )
        .into());
    } else {
        &package.metadata
    };

    let zkprogram = meta
        .get(ZKPROGRAM)
        .ok_or(ZkManifestError::MissingProgramMetadata {
            manifest_path: package.manifest_path.as_str().to_owned(),
            meta: meta.to_owned(),
        })?;

    let input_order = zkprogram
        .get(INPUT_ORDER)
        .ok_or(ZkManifestError::MissingInputOrder {
            manifest_path: package.manifest_path.as_str().to_owned(),
            zkprogram: zkprogram.to_owned(),
        })?;

    let inputs = input_order
        .as_array()
        .ok_or(ZkManifestError::ExpectedArray {
            manifest_path: package.manifest_path.as_str().to_owned(),
            name: INPUT_ORDER.into(),
        })?;

    let (input_order, errs): (
        Vec<Result<String, ZkManifestError>>,
        Vec<Result<String, ZkManifestError>>,
    ) = inputs
        .iter()
        .map(|i| -> Result<String, ZkManifestError> {
            i.as_str()
                .map(|s| s.to_string())
                .ok_or(ZkManifestError::InvalidInput(i.to_owned()))
        })
        .partition(|res| res.is_ok());
    if !errs.is_empty() {
        let errs: Vec<String> = errs
            .into_iter()
            .map(|r| format!("Error: {:?}\n", r.unwrap_err()))
            .collect();
        return Err(ZkManifestError::InvalidInputs {
            manifest_path: package.manifest_path.as_str().to_owned(),
            errs,
        }
        .into());
    }

    Ok(input_order.into_iter().map(Result::unwrap).collect())
}

fn parse_cargo_manifest(image_path: &Path) -> Result<(Package, Vec<String>)> {
    let cargo_manifest_path = fs::canonicalize(image_path.join(CARGO_TOML))?;

    if !cargo_manifest_path.exists() {
        return Err(
            ZkManifestError::MissingManifest(image_path.to_string_lossy().to_string()).into(),
        );
    }

    let meta = MetadataCommand::new()
        .manifest_path(&cargo_manifest_path)
        .no_deps()
        .exec()
        .map_err(|err| ZkManifestError::FailedToLoadManifest {
            manifest_path: cargo_manifest_path.to_string_lossy().to_string(),
            err,
        })?;

    let mut matching: Vec<Package> = meta
        .packages
        .into_iter()
        .filter(|pkg| {
            let std_path: &Path = pkg.manifest_path.as_ref();
            std_path == cargo_manifest_path
        })
        .collect();

    if matching.is_empty() {
        return Err(ZkManifestError::MissingPackage(
            cargo_manifest_path.to_string_lossy().to_string(),
        )
        .into());
    }

    if matching.len() > 1 {
        return Err(ZkManifestError::MultiplePackages(
            cargo_manifest_path.to_string_lossy().to_string(),
        )
        .into());
    }

    let package = matching.pop().unwrap();

    let input_order = parse_cargo_manifest_inputs(&package)?;

    Ok((package, input_order))
}

fn build_zkprogram(
    image_path: &Path,
    keypair: &impl Signer,
    package: &Package,
    input_order: Vec<String>,
) -> Result<ZkProgramManifest> {
    const RISCV_DOCKER_PATH: &str = "target/riscv32im-risc0-zkvm-elf/docker";

    println!("Starting build_zkprogram_manifest");
    println!("image_path: {:?}", image_path);
    println!("cargo_package_name: {}", package.name);

    let binary_path = image_path
        .join(RISCV_DOCKER_PATH)
        .join(format!("{}.bin", package.name));

    println!("Expected binary_path: {:?}", binary_path);

    // Do the build
    let docker_options = DockerOptionsBuilder::default().build().unwrap();

    let options = GuestOptionsBuilder::default()
        .use_docker(docker_options)
        .build()
        .unwrap();

    let r = risc0_build::build_package(&package, "target", options);

    if let Err(err) = r {
        eprintln!("[ERROR] Build failed. Error:\n{}", err);
        return Err(BonsolCliError::BuildFailure(err.to_string()).into());
    }

    println!("Build succeeded. Reading binary from {:?}", binary_path);
    let elf_contents = match fs::read(&binary_path) {
        Ok(bytes) => bytes,
        Err(e) => {
            eprintln!("[ERROR] Failed to read binary at {:?}: {}", binary_path, e);
            return Err(e.into());
        }
    };

    println!("Binary size: {} bytes", elf_contents.len());

    let image_id = compute_image_id(&elf_contents).map_err(|err| {
        eprintln!("[ERROR] Failed to compute image ID: {:?}", err);
        BonsolCliError::FailedToComputeImageId {
            binary_path: binary_path.to_string_lossy().to_string(),
            err,
        }
    })?;

    println!("Computed image_id: {}", image_id);

    let signature = keypair.sign_message(elf_contents.as_slice());
    println!("Signed binary");

    let zkprogram_manifest = ZkProgramManifest {
        name: package.name.clone(),
        binary_path: binary_path
            .to_str()
            .ok_or_else(|| {
                eprintln!("[ERROR] Invalid binary path: {:?}", binary_path);
                ZkManifestError::InvalidBinaryPath
            })?
            .to_string(),
        input_order,
        image_id: image_id.to_string(),
        size: elf_contents.len() as u64,
        signature: signature.to_string(),
    };

    return Ok(zkprogram_manifest);
}
