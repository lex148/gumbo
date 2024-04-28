use crate::templates::TemplateError;
use crate::templates::{
    asset_controller, build, errors, greetings_controller, helpers_mod, inputcss, main, migrations,
    views_mod,
};
use std::path::{Path, PathBuf};
use std::process::{Command, Stdio};
use thiserror::Error;

#[derive(Debug, Error)]
enum InitError {
    #[error("Error Running Cargo: {0}")]
    CargoInitFailed(String),
    #[error("Error Adding Dependencies: {0}")]
    DependenciesFailed(String),
    #[error("{0}")]
    Template(TemplateError),
}

/// Called to to crate a new gumbo project
pub fn run(path: &Path) {
    if let Err(err) = run_inner(path) {
        eprintln!("{err}");
        std::process::exit(1);
    }
}

fn run_inner(path: &Path) -> Result<(), InitError> {
    cargo_init(path)?;

    build::write_template(path).map_err(InitError::Template)?;
    inputcss::write_template(path).map_err(InitError::Template)?;
    asset_controller::write_template(path).map_err(InitError::Template)?;
    greetings_controller::write_template(path).map_err(InitError::Template)?;
    views_mod::write_template(path).map_err(InitError::Template)?;
    helpers_mod::write_template(path).map_err(InitError::Template)?;
    errors::write_template(path).map_err(InitError::Template)?;
    crate::templates::touch(path, "./src/models/mod.rs").map_err(InitError::Template)?;
    migrations::init::write_template(path).map_err(InitError::Template)?;
    main::write_template(path).map_err(InitError::Template)?;
    super::run_rustfmt(path);

    Ok(())
}

/// runs cargo into to crate the project
fn cargo_init(path: &Path) -> Result<(), InitError> {
    // If the cargo project already exists skip this step
    let mut toml_path: PathBuf = path.to_path_buf();
    toml_path.push("Cargo.toml");
    if toml_path.exists() {
        return Ok(());
    }

    // run cargo init path
    let path_str = path
        .to_str()
        .ok_or(InitError::CargoInitFailed("Bad Path".to_owned()))?;
    let out = Command::new("cargo")
        .stdout(Stdio::inherit())
        .stderr(Stdio::inherit())
        .args(["init", path_str, "--name=server"])
        .output();

    match out {
        Err(err) => {
            let err = format!("{err}");
            return Err(InitError::CargoInitFailed(err));
        }
        Ok(out) => {
            if !out.status.success() {
                return Err(InitError::CargoInitFailed("Cargo Error".to_string()));
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
    add_dependencies(path, &["add", "log"])?;
    add_dependencies(path, &["add", "serde"])?;
    add_dependencies(path, &["add", "pretty_env_logger"])?;
    add_dependencies(path, &["add", "yew", "--features=ssr"])?;

    Ok(())
}

/// runs cargo into to crate the project
fn add_dependencies(path: &Path, args: &[&str]) -> Result<(), InitError> {
    let out = Command::new("cargo")
        .stdout(Stdio::inherit())
        .stderr(Stdio::inherit())
        .current_dir(path)
        .args(args)
        .output();
    match out {
        Err(err) => {
            let err = format!("{err}");
            return Err(InitError::DependenciesFailed(err));
        }
        Ok(out) => {
            if !out.status.success() {
                return Err(InitError::DependenciesFailed("Cargo Error".to_string()));
            }
        }
    }
    Ok(())
}
