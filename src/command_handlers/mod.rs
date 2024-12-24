pub(crate) mod convert;
pub(crate) mod database;
pub(crate) mod generate;
pub(crate) mod init;
use std::path::Path;
use std::process::{Command, Stdio};

pub(crate) fn run_rustfmt(root_path: &Path) {
    if let Err(err) = run_rustfmt_inner(root_path) {
        eprintln!("{}", err);
    }
}

fn run_rustfmt_inner(root_path: &Path) -> Result<(), String> {
    // Check if the working directory exists
    if !Path::new(root_path).exists() {
        return Err("Working directory does not exist.".to_string());
    }

    // Execute the rustfmt command
    let result = Command::new("sh")
        .current_dir(root_path)
        .args(["-c", r#"rustfmt --edition 2021 ./**/*.rs"#])
        .stdout(Stdio::inherit())
        .stderr(Stdio::inherit())
        .status();

    match result {
        Ok(status) if status.success() => Ok(()),
        Ok(_) => Err("rustfmt encountered an error or failed to run.".to_string()),
        Err(err) => {
            eprintln!("rustfmt: {:?}", err);
            Err("rustfmt encountered an error or failed to run.".to_string())
        }
    }
}
