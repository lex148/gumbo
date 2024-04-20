use crate::cli::GenerateCommands;
use crate::templates::TemplateError;
use std::fs;
use std::path::{Path, PathBuf};
use thiserror::Error;

mod controller;
mod scaffold;

#[derive(Debug, Error)]
enum GenerateError {
    #[error("Could Not find the root of your project. Are you in a Gumbo project?")]
    NoRootPath,
    #[error("{0}")]
    Template(TemplateError),
}

impl From<TemplateError> for GenerateError {
    fn from(inner: TemplateError) -> Self {
        GenerateError::Template(inner)
    }
}

/// Called to to crate a new gumbo project
pub fn run(cmd: &GenerateCommands) {
    if let Err(err) = run_inner(cmd) {
        eprintln!("{err}");
        std::process::exit(1);
    }
}

fn run_inner(cmd: &GenerateCommands) -> Result<(), GenerateError> {
    match cmd {
        GenerateCommands::Controller { name, actions } => controller::generate(name, actions)?,
        GenerateCommands::Scaffold { name, fields } => scaffold::generate(name, fields)?,
    }
    Ok(())
}

fn get_root_path() -> Result<PathBuf, GenerateError> {
    let path = fs::canonicalize(PathBuf::from("./")).map_err(|_| GenerateError::NoRootPath)?;
    let mut parent: Option<&Path> = Some(&path);
    while let Some(p) = parent {
        let mut toml = path.clone();
        toml.push("Cargo.toml");
        let mut src = path.clone();
        src.push("src");
        if toml.is_file() && src.is_dir() {
            return Ok(p.to_path_buf());
        }
        parent = p.parent();
    }
    Err(GenerateError::NoRootPath)
}
