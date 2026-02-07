use super::get_root_path;
use crate::change::write_to_disk;
use crate::errors::Result;
use crate::fields::Field;
use crate::names::Names;
use crate::templates::migrations;
use std::str::FromStr;

pub(crate) fn generate(name: &str, fields: &[String]) -> Result<()> {
    let root_path = get_root_path()?;
    let names = Names::new(name);

    let fields: std::result::Result<Vec<Field>, _> =
        fields.iter().map(|s| Field::from_str(s)).collect();

    let fields = fields?;

    let changes = [migrations::create_table::write_template(
        &root_path, &names, &fields,
    )?];

    for change in changes.as_ref().iter().flatten() {
        println!("FILE: {:?}", change.file());
    }

    write_to_disk(&root_path, changes.as_slice().iter().flatten())?;

    println!("Migration Generate Completed");
    println!("Migration: {name}");

    crate::command_handlers::run_rustfmt(&root_path);

    Ok(())
}
