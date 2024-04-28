pub(crate) mod generate;
pub(crate) mod init;
use std::env;
use std::path::Path;
use std::process::{Command, Stdio};

pub(crate) fn run_rustfmt(root_path: &Path) {
    if let Err(err) = run_rustfmt_inner(root_path) {
        eprintln!("{}", err);
    }
}

fn run_rustfmt_inner(root_path: &Path) -> Result<(), String> {
    // TODO format rust code that is generated
    Ok(())
    //// Set the desired working directory
    //let working_directory = root_path;

    //// Check if the working directory exists
    //if !Path::new(working_directory).exists() {
    //    return Err("Working directory does not exist.".to_string());
    //}

    //// Change to the desired directory
    //if let Err(e) = env::set_current_dir(working_directory) {
    //    return Err(format!("Failed to change directory: {}", e));
    //}

    //// Execute the rustfmt command
    //let result = Command::new("rustfmt")
    //    .current_dir(root_path)
    //    .args(["--edition", "2021", "./**/*.rs"])
    //    //.stdout(Stdio::null()) // Suppress output
    //    //.stderr(Stdio::null()) // Suppress errors
    //    .stdout(Stdio::inherit())
    //    .stderr(Stdio::inherit())
    //    .status();

    //match result {
    //    Ok(status) if status.success() => Ok(()),
    //    Ok(_) | Err(_) => Err("rustfmt encountered an error or failed to run.".to_string()),
    //}
}
