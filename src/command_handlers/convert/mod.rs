use crate::cli::ConvertCommands;
use std::path::{Path, PathBuf};
use thiserror::Error;

#[derive(Debug, Error)]
enum GenerateError {
    #[error("{0}")]
    Str(&'static str),
}

/// Called to to crate a new gumbo project
pub fn run(cmd: &ConvertCommands) {
    if let Err(err) = run_inner(cmd) {
        eprintln!("{err}");
        std::process::exit(1);
    }
}

fn run_inner(cmd: &ConvertCommands) -> Result<(), GenerateError> {
    match cmd {
        ConvertCommands::Mod2Dir { path } => mod2dir(path)?, //ConvertCommands::Dir2Mod {} => todo!(),
    }
    Ok(())
}

fn mod2dir(path: &Path) -> Result<(), GenerateError> {
    if !path.exists() {
        Err(GenerateError::Str("Path does not exist"))?;
    }
    if !path.is_file() {
        Err(GenerateError::Str("Path is not a file"))?;
    }
    if path.extension().unwrap_or_default().to_string_lossy() != "rs" {
        Err(GenerateError::Str("Expected a rust .rs file"))?;
    }
    let name: String = path
        .file_stem()
        .ok_or(GenerateError::Str(
            "Unable to determain the name of the module",
        ))?
        .to_string_lossy()
        .to_string();
    let name = name.as_str();

    if name == "mod" {
        Err(GenerateError::Str("cannot convert a mod.rs file"))?;
    }

    if name == "main" {
        Err(GenerateError::Str("cannot convert a main.rs file"))?;
    }

    let mut dirpath = path
        .to_path_buf()
        .parent()
        .ok_or(GenerateError::Str("Unable to determain the parent path"))?
        .to_path_buf();
    dirpath.push(name);

    std::fs::create_dir(&dirpath).or(Err(GenerateError::Str("Unable to create directory")))?;

    let mut modpath = dirpath.to_path_buf();
    modpath.push("mod.rs");

    std::fs::rename(path, modpath)
        .or(Err(GenerateError::Str("Unable to move file to directory")))?;

    println!("Converted {name}.rs to {name}/mod.rs");

    Ok(())
}
