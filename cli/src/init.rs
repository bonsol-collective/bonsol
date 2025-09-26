use anyhow::Result;
use std::fs;
use std::path::Path;
use tera::{Context, Tera};

pub fn init_project(project_name: Option<String>, dir: Option<String>) -> Result<()> {
    let pwd = std::env::current_dir()?;

    let project_path = if let Some(ref d) = dir {
        let p = Path::new(d);
        if p.is_relative() {
            pwd.join(p)
        } else {
            p.to_path_buf()
        }
    } else if let Some(name) = &project_name {
        pwd.join(name)
    } else {
        pwd.clone()
    };

    let effective_name = if let Some(n) = project_name {
        n
    } else {
        project_path
            .file_name()
            .map(|s| s.to_string_lossy().to_string())
            .or_else(|| pwd.file_name().map(|s| s.to_string_lossy().to_string()))
            .unwrap_or_else(|| "project".to_string())
    };

    if project_path.exists() && project_path != pwd {
        return Err(anyhow::anyhow!("Project already exists"));
    }

    println!("Creating project skeleton...");

    fs::create_dir_all(project_path.join("src"))?;

    let mut tera = Tera::default();
    tera.add_raw_templates(vec![
        ("Cargo.toml", CARGO_TEMPLATE),
        ("src/main.rs", MAIN_TEMPLATE),
        ("README.md", README_TEMPLATE),
    ])?;

    let mut context = Context::new();
    context.insert("project_name", &effective_name);

    for (template_name, file_name) in &[
        ("Cargo.toml", "Cargo.toml"),
        ("src/main.rs", "src/main.rs"),
        ("README.md", "README.md"),
    ] {
        let content = tera.render(template_name, &context)?;
        fs::write(project_path.join(file_name), content)?;
    }

    println!("Updating Cargo.lock...");

    let status = std::process::Command::new("cargo")
        .arg("generate-lockfile")
        .current_dir(&project_path)
        .status()?;
    if !status.success() {
        return Err(anyhow::anyhow!("`cargo generate-lockfile` failed"));
    }

    println!("Project '{}' initialized successfully!", effective_name);
    Ok(())
}

const CARGO_TEMPLATE: &str = r#"
[package]
name = "{{ project_name }}"
version = "0.1.0"
edition = "2024"

[package.metadata.zkprogram]
input_order = ["Public"]

[workspace]

[dependencies]
risc0-zkvm = {version = "2.3.1", default-features = false, features = ["std"]}

[dependencies.sha2]
git = "https://github.com/risc0/RustCrypto-hashes"
tag = "sha2-v0.10.6-risczero.0"
"#;

const MAIN_TEMPLATE: &str = r#"
use risc0_zkvm::{guest::{env, sha::Impl}, sha::{Sha256}};

fn main() {
    let mut input_1 = Vec::new();
    env::read_slice(&mut input_1);
    let digest = Impl::hash_bytes(&input_1);
    env::commit_slice(digest.as_bytes());
}
"#;

const README_TEMPLATE: &str = r#"
# {{ project_name }}

This is a Bonsol zkprogram, built on risc0
"#;
