use std::fs::File;
use std::io::{Read, Write};
use std::path::Path;

/// Adds a `pub(crate) mod XYZ;` to the list of modules in a given rust file.
pub(crate) fn append_module(
    root_path: &Path,
    modpath: &Path,
    name: &str,
) -> Result<(), std::io::Error> {
    let mut full_path = root_path.to_path_buf();
    full_path.push(modpath);

    if full_path.is_file() {
        let mut file = File::options().read(true).open(&full_path)?;
        let mut content = String::default();
        file.read_to_string(&mut content)?;
        let target = format!("mod {name}");
        if content.contains(&target) {
            return Ok(());
        }
    }
    let line = format!("\npub(crate) mod {name};");
    let mut file = File::options().append(true).create(true).open(&full_path)?;
    file.write_all(line.as_bytes())?;
    Ok(())
}
