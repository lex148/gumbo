use crate::templates::TemplateError;
use crate::templates::{
    asset_controller, auth_controller, build, docker, errors, greetings_controller, helpers_mod,
    inputcss, main, migrations, models_session, views_mod,
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
    auth_controller::write_template(path).map_err(InitError::Template)?;
    greetings_controller::write_template(path).map_err(InitError::Template)?;
    views_mod::write_template(path).map_err(InitError::Template)?;
    helpers_mod::write_template(path).map_err(InitError::Template)?;
    errors::write_template(path).map_err(InitError::Template)?;
    crate::templates::touch(path, "./src/models/mod.rs").map_err(InitError::Template)?;
    models_session::write_template(path).map_err(InitError::Template)?;
    migrations::init::write_template(path).map_err(InitError::Template)?;
    docker::write_template(path).map_err(InitError::Template)?;
    main::write_template(path).map_err(InitError::Template)?;
    super::run_rustfmt(path);
    crate::command_handlers::generate::dotenv::write_template(path).map_err(InitError::Template)?;

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
    add_dependencies(path, &["add", "actix-files"])?;
    add_dependencies(path, &["add", "log"])?;
    add_dependencies(path, &["add", "serde"])?;
    add_dependencies(path, &["add", "pretty_env_logger"])?;
    add_dependencies(path, &["add", "yew", "--features=ssr"])?;
    add_dependencies(path, &["add", "aes-gcm"])?;
    add_dependencies(path, &["add", "base64"])?;
    add_dependencies(path, &["add", "bincode"])?;
    add_dependencies(path, &["add", "dotenvy"])?;
    add_dependencies(path, &["add", "futures"])?;
    add_dependencies(path, &["add", "oauth2"])?;
    add_dependencies(path, &["add", "rand"])?;
    // version 0.11 to match auth2
    add_dependencies(path, &["add", "reqwest@0.11", "--features=json"])?;

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
