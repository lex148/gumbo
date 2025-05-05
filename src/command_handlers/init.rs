use crate::change::{Change, write_to_disk};
use crate::errors::{GumboError, Result};
use crate::templates::{
    asset_controller, auth_controller, build, cargo_config, docker, errors, greetings_controller,
    inputcss, main, migrations, views_mod,
};
use clap::ArgMatches;
use std::collections::HashSet;
use std::path::{Path, PathBuf};
use std::process::{Command, Stdio};

/// Called to to crate a new gumbo project
pub fn run(args: &ArgMatches) {
    let path = args.get_one::<PathBuf>("path").unwrap();
    let welds_only = args.get_one::<bool>("welds_only").unwrap();

    let mut backends = std::collections::HashSet::new();
    let mssql = args.get_one::<bool>("mssql").unwrap();
    let mysql = args.get_one::<bool>("mysql").unwrap();
    let postgres = args.get_one::<bool>("postgres").unwrap();
    let sqlite = args.get_one::<bool>("sqlite").unwrap();

    if *mysql {
        backends.insert("mysql");
    }
    if *mssql {
        backends.insert("mssql");
    }
    if *postgres {
        backends.insert("postgres");
    }
    if *sqlite {
        backends.insert("sqlite");
    }

    //pub fn run(path: &Path, welds_only: bool, backends: HashSet<&'static str>) {
    if *welds_only {
        if let Err(err) = run_inner_welds_only(path, backends) {
            eprintln!("{err}");
            std::process::exit(1);
        }
    } else if let Err(err) = run_inner(path, backends) {
        eprintln!("{err}");
        std::process::exit(1);
    }
}

fn run_inner_welds_only(rootpath: &Path, mut backends: HashSet<&'static str>) -> Result<()> {
    cargo_init(rootpath, None)?;

    add_dependencies(rootpath, &["add", "welds", "--features=migrations"])?;
    backends.extend(env_backends());
    if backends.is_empty() {
        backends.insert("sqlite");
    }
    for b in backends.iter() {
        let features = format!("--features={b}");
        add_dependencies(rootpath, &["add", "welds", &features])?;
    }

    add_dependencies(
        rootpath,
        &[
            "add",
            "sqlx",
            "--features=runtime-tokio,tls-rustls,uuid,chrono",
        ],
    )?;
    add_dependencies(rootpath, &["add", "chrono"])?;
    add_dependencies(rootpath, &["add", "log"])?;
    add_dependencies(rootpath, &["add", "pretty_env_logger"])?;
    add_dependencies(
        rootpath,
        &["add", "tokio", "--features=rt-multi-thread,macros"],
    )?;
    add_dependencies(rootpath, &["add", "uuid", "--features=v4,serde"])?;
    add_dependencies(rootpath, &["add", "dotenvy"])?;

    let mut changes = vec![
        //errors::write_template()?,
        vec![
            Change::new(
                "./src/errors/mod.rs",
                "pub type Result<T> = welds::errors::Result<T>;",
            )?
            .append(),
        ],
        vec![Change::new("./src/models/mod.rs", "")?.append()],
        migrations::init::write_template()?,
        main::write_template_welds_only()?,
        crate::command_handlers::generate::dotenv::write_template_lite()?,
        vec![Change::new("./.gitignore", "\n.env\n*.sqlite\n")?.append()],
    ];

    if backends.contains(&"sqlite") {
        changes.push(vec![Change::new("./dev.sqlite", "")?.append()]);
    }

    for change in changes.iter().flatten() {
        println!("CREATING: {:?}", change.file());
    }

    for change in changes.iter().flatten() {
        write_to_disk(rootpath, change)?;
    }

    super::run_rustfmt(rootpath);

    //println!();
    //println!(
    //    "To start developing, go to your project and run: \"cargo watch -d 0.0 -w ./src -x run\""
    //);

    Ok(())
}

fn run_inner(rootpath: &Path, mut backends: HashSet<&'static str>) -> Result<()> {
    cargo_init(rootpath, Some("server"))?;

    add_dependencies(rootpath, &["add", "welds", "--features=migrations"])?;
    backends.extend(env_backends());
    if backends.is_empty() {
        backends.insert("sqlite");
    }
    for b in &backends {
        let features = format!("--features={b}");
        add_dependencies(rootpath, &["add", "welds", &features])?;
    }

    add_dependencies(
        rootpath,
        &[
            "add",
            "sqlx",
            "--features=runtime-tokio,tls-rustls,uuid,chrono",
        ],
    )?;
    add_dependencies(rootpath, &["add", "chrono"])?;
    add_dependencies(rootpath, &["add", "thiserror"])?;
    add_dependencies(rootpath, &["add", "actix-web"])?;
    add_dependencies(rootpath, &["add", "actix-files"])?;
    add_dependencies(rootpath, &["add", "log"])?;
    add_dependencies(rootpath, &["add", "serde"])?;
    add_dependencies(rootpath, &["add", "pretty_env_logger"])?;
    add_dependencies(rootpath, &["add", "yew", "--features=ssr"])?;
    add_dependencies(rootpath, &["add", "tokio", "--features=sync"])?;
    add_dependencies(rootpath, &["add", "uuid", "--features=v4,serde"])?;
    add_dependencies(rootpath, &["add", "aes-gcm"])?;
    add_dependencies(rootpath, &["add", "base64"])?;
    add_dependencies(rootpath, &["add", "bincode"])?;
    add_dependencies(rootpath, &["add", "dotenvy"])?;
    add_dependencies(rootpath, &["add", "futures"])?;
    add_dependencies(rootpath, &["add", "oauth2"])?;
    add_dependencies(rootpath, &["add", "rand"])?;
    add_dependencies(
        rootpath,
        &["add", "gumbo-lib", "--features=sessions,turbo-streams"],
    )?;
    // version 0.11 to match auth2
    add_dependencies(rootpath, &["add", "reqwest@0.11", "--features=json"])?;

    let full = std::fs::canonicalize(rootpath)?;
    let name: String = full
        .file_name()
        .map(|s| s.to_string_lossy().to_string())
        .unwrap_or("Site".to_string());

    let logo = include_bytes!("../templates/gumbo.webp");

    let mut changes = vec![
        build::write_template()?,
        inputcss::write_template()?,
        asset_controller::write_template()?,
        auth_controller::write_template()?,
        greetings_controller::write_template()?,
        views_mod::write_template(&name)?,
        errors::write_template()?,
        vec![Change::new("./src/models/mod.rs", "")?.append()],
        vec![Change::new("./src/assets/.gitkeep", "")?.append()],
        vec![Change::new("./src/assets/js/.gitkeep", "")?.append()],
        vec![Change::new("./src/assets/gumbo.webp", logo.as_slice())?.append()],
        migrations::init::write_template()?,
        docker::write_template()?,
        cargo_config::write_template()?,
        main::write_template()?,
        crate::command_handlers::generate::dotenv::write_template()?,
        vec![Change::new("./.gitignore", "\n.env\n*.sqlite\n")?.append()],
    ];

    if backends.contains(&"sqlite") {
        changes.push(vec![Change::new("./dev.sqlite", "")?.append()]);
    }

    for change in changes.iter().flatten() {
        println!("CREATING: {:?}", change.file());
    }

    for change in changes.iter().flatten() {
        write_to_disk(rootpath, change)?;
    }

    super::run_rustfmt(rootpath);

    println!();
    println!(
        "To start developing, go to your project and run: \"cargo watch -d 0.0 -w ./src -x run\""
    );

    Ok(())
}

/// runs cargo into to crate the project
fn cargo_init(path: &Path, name: Option<&str>) -> Result<()> {
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

    let out = match name {
        Some(name) => {
            let namearg = format!("--name={}", name);
            Command::new("cargo")
                .stdout(Stdio::inherit())
                .stderr(Stdio::inherit())
                .args(["init", path_str, &namearg])
                .output()
        }
        None => Command::new("cargo")
            .stdout(Stdio::inherit())
            .stderr(Stdio::inherit())
            .args(["init", path_str])
            .output(),
    };

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

    Ok(())
}

pub(crate) fn env_backends() -> HashSet<&'static str> {
    let mut set = std::collections::HashSet::new();
    let url = std::env::var("DATABASE_URL").ok().unwrap_or_default();

    if url.starts_with("postgresql:") {
        set.insert("postgres");
    }
    if url.starts_with("postgres:") {
        set.insert("postgres");
    }
    if url.starts_with("mysql:") {
        set.insert("mysql");
    }
    if url.starts_with("sqlite:") {
        set.insert("sqlite");
    }
    set
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
