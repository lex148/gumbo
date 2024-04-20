use super::get_root_path;
use crate::command_handlers::generate::GenerateError;
use crate::templates;
use std::str::FromStr;
use templates::controller;
use templates::controller::Action;

pub(crate) fn generate(name: &str, actions: &[String]) -> Result<(), GenerateError> {
    let name = name.trim().to_lowercase();
    let root_path = get_root_path()?;

    let actions: Vec<Action> = actions
        .iter()
        .filter_map(|x| Action::from_str(x.as_str()).ok())
        .collect();

    controller::write_template(&root_path, &name, &actions)?;

    Ok(())
}
