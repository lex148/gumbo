use crate::change::{write_to_disk, Change};
use crate::errors::{GumboError, Result};
use crate::templates::{
    asset_controller, auth_controller, build, docker, errors, greetings_controller, inputcss, main,
    migrations, views_mod,
};
use std::path::{Path, PathBuf};
use std::process::{Command, Stdio};

/// Called to to crate a new gumbo project
pub fn run(path: &Path) {
    if let Err(err) = run_inner(path) {
        eprintln!("{err}");
        std::process::exit(1);
    }
}

fn run_inner(rootpath: &Path) -> Result<()> {
    cargo_init(rootpath)?;

    let full = std::fs::canonicalize(rootpath)?;
    let name: String = full
        .file_name()
        .map(|s| s.to_string_lossy().to_string())
        .unwrap_or("Site".to_string());

    let logo = include_bytes!("../templates/gumbo.webp");

    let changes = [
        build::write_template()?,
        inputcss::write_template()?,
        asset_controller::write_template()?,
        auth_controller::write_template()?,
        greetings_controller::write_template()?,
        views_mod::write_template(&name)?,
        errors::write_template()?,
        vec![Change::new("./dev.sqlite", "")?.append()],
        vec![Change::new("./src/models/mod.rs", "")?.append()],
        vec![Change::new("./src/assets/.gitkeep", "")?.append()],
        vec![Change::new("./src/assets/js/.gitkeep", "")?.append()],
        vec![Change::new("./src/assets/gumbo.webp", logo.as_slice())?.append()],
        migrations::init::write_template()?,
        docker::write_template()?,
        main::write_template()?,
        crate::command_handlers::generate::dotenv::write_template()?,
        vec![Change::new("./.gitignore", "\n.env\n*.sqlite\n")?.append()],
    ];

    for change in changes.as_ref().iter().flatten() {
        println!("CREATING: {:?}", change.file());
    }

    for change in changes.as_ref().iter().flatten() {
        write_to_disk(rootpath, change)?;
    }

    super::run_rustfmt(rootpath);

    Ok(())
}

/// runs cargo into to crate the project
fn cargo_init(path: &Path) -> Result<()> {
    // If the cargo project already exists skip this step
    let mut toml_path: PathBuf = path.to_path_buf();
    toml_path.push("Cargo.toml");
    if toml_path.exists() {
        return Ok(());
    }

    // run cargo init path
    let path_str = path
        .to_str()
        .ok_or(GumboError::CargoInitFailed("Bad Path".to_owned()))?;
    let out = Command::new("cargo")
        .stdout(Stdio::inherit())
        .stderr(Stdio::inherit())
        .args(["init", path_str, "--name=server"])
        .output();

    match out {
        Err(err) => {
            let err = format!("{err}");
            return Err(GumboError::CargoInitFailed(err));
        }
        Ok(out) => {
            if !out.status.success() {
                return Err(GumboError::CargoInitFailed("Cargo Error".to_string()));
            }
        }
    }

    add_dependencies(path, &["add", "welds", "--features=sqlite,migrations"])?;
    add_dependencies(
        path,
        &["add", "sqlx", "--features=runtime-tokio,tls-rustls"],
    )?;
    add_dependencies(path, &["add", "thiserror"])?;
    add_dependencies(path, &["add", "actix-web"])?;
    add_dependencies(path, &["add", "actix-files"])?;
    add_dependencies(path, &["add", "log"])?;
    add_dependencies(path, &["add", "serde"])?;
    add_dependencies(path, &["add", "pretty_env_logger"])?;
    add_dependencies(path, &["add", "yew", "--features=ssr"])?;
    add_dependencies(path, &["add", "tokio", "--features=sync"])?;
    add_dependencies(path, &["add", "aes-gcm"])?;
    add_dependencies(path, &["add", "base64"])?;
    add_dependencies(path, &["add", "bincode"])?;
    add_dependencies(path, &["add", "dotenvy"])?;
    add_dependencies(path, &["add", "futures"])?;
    add_dependencies(path, &["add", "oauth2"])?;
    add_dependencies(path, &["add", "rand"])?;
    add_dependencies(
        path,
        &["add", "gumbo-lib", "--features=sessions,turbo-streams"],
    )?;
    // version 0.11 to match auth2
    add_dependencies(path, &["add", "reqwest@0.11", "--features=json"])?;

    Ok(())
}

/// runs cargo into to crate the project
fn add_dependencies(path: &Path, args: &[&str]) -> Result<()> {
    let out = Command::new("cargo")
        .stdout(Stdio::inherit())
        .stderr(Stdio::inherit())
        .current_dir(path)
        .args(args)
        .output();
    match out {
        Err(err) => {
            let err = format!("{err}");
            return Err(GumboError::DependenciesFailed(err));
        }
        Ok(out) => {
            if !out.status.success() {
                return Err(GumboError::DependenciesFailed("Cargo Error".to_string()));
            }
        }
    }
    Ok(())
}
