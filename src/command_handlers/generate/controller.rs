use super::get_root_path;
use crate::action::Action;
use crate::change::{Change, write_to_disk};
use crate::errors::Result;
use crate::names::Names;
use crate::templates;
use std::str::FromStr;
use templates::controller;
use templates::view;

pub(crate) fn generate(name: &str, actions: &[String], no_views: bool) -> Result<()> {
    let name = name.trim().to_lowercase();
    let root_path = get_root_path()?;
    let names = Names::new(&name);

    let actions: Vec<Action> = actions
        .iter()
        .filter_map(|x| Action::from_str(x.as_str()).ok())
        .collect();

    let mut changes: Vec<Vec<Change>> = vec![controller::write_template(&names, &actions)?];

    let view_actions: Vec<_> = actions
        .iter()
        .filter(|a| a.method == "get")
        .cloned()
        .collect();
    if !view_actions.is_empty() && no_views == false {
        changes.push(view::write_empty_templates(&names, &view_actions)?);
    }

    for change in changes.iter().flatten() {
        println!("FILE: {:?}", change.file());
    }

    for change in changes.iter().flatten() {
        write_to_disk(&root_path, change)?;
    }

    println!("Controller Generate Completed");
    crate::command_handlers::run_rustfmt(&root_path);

    Ok(())
}
