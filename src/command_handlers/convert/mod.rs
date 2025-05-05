//use crate::cli::ConvertCommands;
use clap::ArgMatches;
use std::path::{Path, PathBuf};
use thiserror::Error;

#[derive(Debug, Error)]
enum GenerateError {
    #[error("{0}")]
    Str(&'static str),
}

/// Called to to crate a new gumbo project
pub fn run(args: &ArgMatches) {
    if let Err(err) = run_inner(args) {
        eprintln!("{err}");
        std::process::exit(1);
    }
}

fn run_inner(args: &ArgMatches) -> Result<(), GenerateError> {
    match args.subcommand() {
        Some(("mod2dir", sub_m)) => {
            let path = sub_m.get_one::<PathBuf>("path").unwrap();
            mod2dir(path)?;
        }
        Some(("dir2mod", sub_m)) => {
            let path = sub_m.get_one::<PathBuf>("path").unwrap();
            dir2mod(path)?;
        }
        _ => unreachable!(),
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

fn dir2mod(path: &Path) -> Result<(), GenerateError> {
    if !path.exists() {
        Err(GenerateError::Str("Path does not exist"))?;
    }
    if !path.is_dir() {
        Err(GenerateError::Str("Path is not a directory"))?;
    }

    let files = std::fs::read_dir(path).or(Err(GenerateError::Str("Path is not a directory")))?;
    let mut files: Vec<_> = files.collect();
    if files.len() != 1 {
        Err(GenerateError::Str(
            "Can only convert back to mod file if directory only contains mod.rs file",
        ))?;
    }
    let contents: PathBuf = files.pop().unwrap().unwrap().path();

    if !contents.as_path().is_file() {
        Err(GenerateError::Str(
            "Can only convert back to mod file if directory only contains mod.rs file",
        ))?;
    }

    if contents.file_name().unwrap().to_string_lossy() != "mod.rs" {
        Err(GenerateError::Str(
            "Can only convert back to mod file if directory only contains mod.rs file",
        ))?;
    }

    let name = path.file_name().unwrap().to_string_lossy().to_string();
    let name = name.as_str();
    let mut modpath = path.parent().unwrap().to_path_buf();
    modpath.push(format!("{name}.rs"));

    std::fs::rename(contents, modpath).or(Err(GenerateError::Str("Unable to move mod.rs")))?;

    std::fs::remove_dir(path).or(Err(GenerateError::Str(
        "Unable to cleanup the directory: {name}",
    )))?;

    Ok(())
}
