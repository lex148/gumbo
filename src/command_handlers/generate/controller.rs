use super::get_root_path;
use crate::action::Action;
use crate::command_handlers::generate::GenerateError;
use crate::names::Names;
use crate::templates;
use std::str::FromStr;
use templates::controller;
use templates::view;

pub(crate) fn generate(name: &str, actions: &[String]) -> Result<(), GenerateError> {
    let name = name.trim().to_lowercase();
    let root_path = get_root_path()?;
    let names = Names::new(&name);

    let actions: Vec<Action> = actions
        .iter()
        .filter_map(|x| Action::from_str(x.as_str()).ok())
        .collect();

    controller::write_template(&root_path, &names, &actions)?;

    let view_actions: Vec<_> = actions
        .iter()
        .filter(|a| a.method == "get")
        .cloned()
        .collect();

    view::write_empty_templates(&root_path, &names, &view_actions)?;

    crate::command_handlers::run_rustfmt(&root_path);

    println!("Controller Generate Completed");

    Ok(())
}
