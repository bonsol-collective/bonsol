use std::env;
use std::fs;
use std::path::{Path, PathBuf};
use std::process::Command;
const FLATC_VERSION: &str = "24.3.25";
fn main() {
    // Check if flatc is installed.
    let flatc_version = Command::new("flatc")
        .arg("--version")
        .output()
        .ok()
        .and_then(|output| String::from_utf8(output.stdout).ok());
    if let Some(version) = flatc_version {
        if !version.contains(FLATC_VERSION) {
            panic!(
                "Expected flatc version {}, found {}",
                FLATC_VERSION, version
            );
        }
    } else {
        panic!(
            "flatc not found. Please install flatc version {}.",
            FLATC_VERSION
        );
    }
    // Define schema directory and target directory for generated Rust code.
    let schema_dir = Path::new("flatbuffers");

    // Collect all .fbs files in the schema directory.
    let file_list: Vec<_> = fs::read_dir(schema_dir)
        .expect("Schema directory not found")
        .filter_map(|entry| {
            entry.ok().and_then(|e| {
                let path = e.path();
                if path.extension().and_then(|ext| ext.to_str()) == Some("fbs") {
                    Some(path)
                } else {
                    None
                }
            })
        })
        .collect();

    // Build flatc arguments.
    let out_dir = std::env::var("OUT_DIR").unwrap();
    let mut args = vec![
        "--gen-mutable",
        "--gen-object-api",
        "--reflect-names",
        "--rust",
        "-o",
        &out_dir,
    ];

    // Add schema file paths to the arguments.
    args.extend(file_list.iter().map(|path| path.to_str().unwrap()));

    // Execute flatc.
    let compile_status = Command::new("flatc")
        .args(&args)
        .status()
        .expect("Failed to execute flatc command");

    assert!(
        compile_status.success(),
        "flatc failed to compile schema files: {:?}",
        file_list
    );

    // Set an environment variable with the generated source path, stripping "src/" if present.
    // let generated_path = generated_src.strip_prefix("src").unwrap_or(&generated_src);
    // println!("cargo:rustc-env=GENERATED_SRC={}", generated_path.display());
    // Instruct Cargo to re-run this script if any files in the schema directory change.
    for file in file_list {
        //        println!("cargo:rerun-if-changed={}", file.display());
    }
}
