use super::get_root_path;
use crate::command_handlers::generate::GenerateError;
use crate::fields::Field;
use crate::names::Names;
use crate::templates::{controller, migrations, models, view};
use std::str::FromStr;

pub(crate) fn generate(name: &str, fields: &[String]) -> Result<(), GenerateError> {
    let root_path = get_root_path()?;
    let names = Names::new(name);

    let fields: Result<Vec<Field>, _> = fields.iter().map(|s| Field::from_str(s)).collect();

    //TODO: display a better message why it can scaffold
    let fields = fields.expect("Error: unknown type in fields");

    models::write_template(&root_path, &names, &fields)?;
    migrations::create_table::write_template(&root_path, &names, &fields)?;

    controller::write_crud_templates(&root_path, &names, &fields)?;
    view::write_crud_templates(&root_path, &names, &fields)?;
    crate::command_handlers::run_rustfmt(&root_path);

    println!("Scaffold Completed");
    println!("Resorse: {name}");

    Ok(())
}
