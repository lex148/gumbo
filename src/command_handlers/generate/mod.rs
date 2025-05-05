use std::fs;
use std::path::{Path, PathBuf};

mod controller;
pub(crate) mod dotenv;
mod migration;
mod model;
mod scaffold;
use crate::errors::Result;
use clap::ArgMatches;

/// Called to to crate a new gumbo project
pub fn run(args: &ArgMatches) {
    if let Err(err) = run_inner(args) {
        eprintln!("{err}");
        std::process::exit(1);
    }
}

fn run_inner(args: &ArgMatches) -> Result<()> {
    match args.subcommand() {
        Some(("controller", sub_m)) => {
            let name: &String = sub_m.get_one::<String>("name").unwrap();
            let actions = sub_m.get_many::<String>("actions").unwrap();
            let actions: Vec<String> = actions.into_iter().cloned().collect();
            let no_views = sub_m.get_one::<bool>("no_views").unwrap();
            controller::generate(name, &actions, *no_views)?;
        }
        Some(("scaffold", sub_m)) => {
            let name: &String = sub_m.get_one::<String>("name").unwrap();
            let fields = sub_m.get_many::<String>("fields").unwrap();
            let fields: Vec<String> = fields.into_iter().cloned().collect();
            scaffold::generate(name, &fields)?
        }
        Some(("migration", sub_m)) => {
            let name: &String = sub_m.get_one::<String>("name").unwrap();
            let fields = sub_m.get_many::<String>("fields").unwrap();
            let fields: Vec<String> = fields.into_iter().cloned().collect();
            migration::generate(name, &fields)?
        }
        Some(("model", sub_m)) => {
            let name: &String = sub_m.get_one::<String>("name").unwrap();
            let fields = sub_m.get_many::<String>("fields").unwrap();
            let fields: Vec<String> = fields.into_iter().cloned().collect();
            let no_migration = sub_m.get_one::<bool>("no_migration").unwrap();
            model::generate(name, &fields, *no_migration)?
        }
        Some(("env", _sub_m)) => dotenv::generate()?,
        _ => unreachable!(),
    }

    Ok(())
}

pub(crate) fn get_root_path() -> crate::errors::Result<PathBuf> {
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
