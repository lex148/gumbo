pub(crate) mod create_table;
pub(crate) mod init;
use crate::templates::TemplateError;
use chrono::{DateTime, Datelike, Local, Timelike};
use std::fs::File;
use std::io::{Read, Seek, Write};
use std::path::Path;

pub(crate) fn timestamp() -> String {
    // Get the current local date and time
    let now: DateTime<Local> = Local::now();

    // Format the date and time into the specified format YYYYMMDDHHMMSS
    let formatted_time = format!(
        "{:04}{:02}{:02}{:02}{:02}{:02}",
        now.year(),
        now.month(),
        now.day(),
        now.hour(),
        now.minute(),
        now.second()
    );

    formatted_time
}

/// Appends a migration to the list of migrations that should be ran
pub(crate) fn migration_list_append(
    root_path: &Path,
    migration_name: &str,
) -> Result<(), TemplateError> {
    let mut path = root_path.to_path_buf();
    path.push("./src/migrations/mod.rs");

    let mut file = File::options().write(true).read(true).open(&path)?;
    file.rewind()?;
    let mut content = String::default();
    file.read_to_string(&mut content)?;
    file.rewind()?;

    let marker = "/* MIGRATION LIST MARKER */";
    let list_content = find_list_content(&content, marker).unwrap_or_default();
    let to_remove = format!("{}{}", list_content, marker);

    let lines: Vec<String> = list_content
        .split('\n')
        .map(|l| l.trim().trim_end_matches(','))
        .filter(|l| !l.is_empty())
        .chain([migration_name])
        .map(|l| format!("        {l},"))
        .collect();
    let lines = lines.join("\n");

    // Replace the marker with `to_insert + marker`
    let modified_content = content.replace(&to_remove, &format!("\n{}{}", lines, marker));

    //let mut file = File::options().truncate(true).create(true).open(&path)?;
    file.write_all(modified_content.as_bytes())?;
    Ok(())
}

/// returns everything between the opening !vec[ and the marker
fn find_list_content<'a>(input: &'a str, marker: &str) -> Option<&'a str> {
    if let Some(marker_index) = input.find(marker) {
        if let Some(last_open_bracket_index) = input[..marker_index].rfind('[') {
            // Return the slice of content between the last '[' and the marker
            return Some(&input[last_open_bracket_index + 1..marker_index]);
        }
    }
    None // Return None if the marker or '[' is not found
}
