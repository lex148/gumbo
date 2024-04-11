use super::{ensure_directory_exists, TemplateError};
use std::fs::File;
use std::io::Write;
use std::path::Path;

pub(crate) fn write_template(root_path: &Path) -> Result<(), TemplateError> {
    let mut path = root_path.to_path_buf();
    path.push("./build.rs");
    ensure_directory_exists(&path)?;
    let mut file = File::options()
        .write(true)
        .truncate(true)
        .create(true)
        .open(&path)?;

    file.write_all(CODE.as_bytes())?;

    Ok(())
}

static CODE: &str = r#"use std::env;
use std::path::Path;
use std::process::Command;

fn main() {
    // Set the directory where the command should be run
    let out_dir = env::var("OUT_DIR").unwrap();
    let out_dir = Path::new(&out_dir);
    //let out_dir = out_dir.parent().unwrap().parent().unwrap().parent().unwrap();
    let out_file = out_dir.join("app.css");

    let manifest_dir = env::var("CARGO_MANIFEST_DIR").expect("CARGO_MANIFEST_DIR not set");
    let manifest_dir = Path::new(&manifest_dir);
    let input_css = manifest_dir.join("src").join("input.css");

    // Determine the build profile
    let profile = env::var("PROFILE").unwrap();

    // Prepare the base command and arguments
    let mut command_args = vec![
        "-i",
        input_css.to_str().unwrap(),
        "-o",
        out_file.to_str().unwrap(),
    ];

    // If the build profile is "release", add the "-m" flag
    if profile == "release" {
        command_args.push("-m");
    }

    // Construct and run the command
    let tailwind_command = Command::new("tailwindcss")
        .current_dir(manifest_dir)
        .args(&command_args)
        .output()
        .expect("Failed to execute tailwindcss command");

    if !tailwind_command.status.success() {
        panic!(
            "Failed to build CSS with Tailwind: {}",
            String::from_utf8_lossy(&tailwind_command.stderr)
        );
    }

    println!("Cargo:rerun-if-changed={}", input_css.to_str().unwrap());
}

"#;
