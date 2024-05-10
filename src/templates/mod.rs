use std::path::{Path, PathBuf};
use thiserror::Error;

pub(crate) mod asset_controller;
pub(crate) mod auth_controller;
pub(crate) mod build;
pub(crate) mod controller;
pub(crate) mod docker;
pub(crate) mod errors;
pub(crate) mod greetings_controller;
pub(crate) mod helpers_mod;
pub(crate) mod inputcss;
pub(crate) mod main;
pub(crate) mod migrations;
pub(crate) mod models;
pub(crate) mod models_session;
pub(crate) mod modrs;
pub(crate) mod view;
pub(crate) mod views_mod;

#[derive(Debug, Error)]
pub enum TemplateError {
    #[error("This File {0} already exists")]
    FileAlreadyExists(PathBuf),
    #[error("Unable to create Path: {0}")]
    UnableToCreatePath(PathBuf),
    #[error("File Error: {0}")]
    IoErr(std::io::Error),
    #[error("FormatError: {0}")]
    FormatError(String),
}

impl From<std::io::Error> for TemplateError {
    fn from(inner: std::io::Error) -> Self {
        TemplateError::IoErr(inner)
    }
}

pub(crate) fn ensure_directory_exists(path: &Path) -> Result<(), TemplateError> {
    if let Some(dir_path) = path.parent() {
        let r = std::fs::create_dir_all(dir_path);
        if r.is_err() {
            return Err(TemplateError::UnableToCreatePath(path.to_path_buf()));
        }
    }
    Ok(())
}

pub(crate) fn try_format(path: &Path) -> Result<(), TemplateError> {
    use std::process::{Command, Stdio};
    let out = Command::new("rustfmt")
        .stdout(Stdio::inherit())
        .stderr(Stdio::inherit())
        .current_dir(path)
        .args(["--edition", "2021", path.to_str().unwrap()])
        .output();
    match out {
        Err(err) => {
            let err = format!("{err}");
            return Err(TemplateError::FormatError(err));
        }
        Ok(out) => {
            if !out.status.success() {
                return Err(TemplateError::FormatError("rustfmt Error".to_string()));
            }
        }
    }
    Ok(())
}

pub(crate) fn touch(root: &Path, sub: &str) -> Result<(), TemplateError> {
    let mut path = root.to_path_buf();
    path.push(sub);
    ensure_directory_exists(&path)?;
    // Open the file in read-write mode without truncating it, creating it if it does not exist.
    use std::fs::File;
    let _file = File::options().append(true).create(true).open(path)?;
    Ok(())
}
