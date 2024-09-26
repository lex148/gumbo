use crate::cli::GenerateCommands;
use std::fs;
use std::path::{Path, PathBuf};

mod controller;
pub(crate) mod dotenv;
mod model;
mod scaffold;
use crate::errors::Result;

/// Called to to crate a new gumbo project
pub fn run(cmd: &GenerateCommands) {
    if let Err(err) = run_inner(cmd) {
        eprintln!("{err}");
        std::process::exit(1);
    }
}

fn run_inner(cmd: &GenerateCommands) -> Result<()> {
    match cmd {
        GenerateCommands::Controller { name, actions } => controller::generate(name, actions)?,
        GenerateCommands::Scaffold { name, fields } => scaffold::generate(name, fields)?,
        GenerateCommands::Model { name, fields } => model::generate(name, fields)?,
        GenerateCommands::Env {} => dotenv::generate()?,
    }
    Ok(())
}

fn get_root_path() -> crate::errors::Result<PathBuf> {
    let path = fs::canonicalize(PathBuf::from("./"))
        .or(Err(crate::errors::GumboError::InvalidRootPath))?;
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
    Err(crate::errors::GumboError::InvalidRootPath)
}
