use crate::change::Change;
use crate::errors::Result;
use std::process::Command;

pub(crate) fn write_template() -> Result<Vec<Change>> {
    if !is_command_available("mold") {
        return Ok(Vec::default());
    }

    Ok(vec![Change::new(
        "./.cargo/config.toml",
        CARGO_CONFIG.trim(),
    )?])
}

fn is_command_available(command: &str) -> bool {
    Command::new(command)
        .arg("--version")
        .output()
        .map(|output| output.status.success())
        .unwrap_or(false)
}

static CARGO_CONFIG: &str = r#"
[target.x86_64-unknown-linux-gnu]
rustflags = ["-C", "link-arg=-fuse-ld=mold"]
"#;
