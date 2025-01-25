use crate::change::Change;
use crate::errors::Result;

pub(crate) fn write_template() -> Result<Vec<Change>> {
    let c = Change::new("./build.rs", CODE)?;
    Ok(vec![c])
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

    // Make sure we have a copy of tailwind downloaded
    download_tailwind(manifest_dir);

    // Construct and run the command
    let tailwind_command = Command::new("./target/tailwindcss")
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

fn download_tailwind(manifest_dir: &Path) {
    // Determine the appropriate Tailwind CSS binary URL based on the OS
    #[cfg(target_os = "macos")]
    let os = "macos";
    #[cfg(target_os = "linux")]
    let os = "linux";
    #[cfg(target_os = "windows")]
    let os = "windows";

    #[cfg(target_arch = "aarch64")]
    let arch = "arm64";
    #[cfg(target_arch = "x86_64")]
    let arch = "x64";

    let version = "v4.0.0";

    let url = format!("https://github.com/tailwindlabs/tailwindcss/releases/download/{version}/tailwindcss-{os}-{arch}");

    let command = format!("test -e ./target/tailwindcss || curl -sL {url} -o ./target/tailwindcss");

    // download the tailwindcss binary if missing
    let _tailwind_command = Command::new("sh")
        .current_dir(manifest_dir)
        .args(["-c", &command])
        .output()
        .expect("Unable to download tailwindcss");

    // download the tailwindcss binary if missing
    let _tailwind_command = Command::new("sh")
        .current_dir(manifest_dir)
        .args(["-c", "chmod +x ./target/tailwindcss"])
        .output();
}

"#;
