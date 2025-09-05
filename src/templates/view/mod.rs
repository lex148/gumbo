use crate::action::Action;
use crate::change::Change;
use crate::errors::Result;
use crate::fields::Field;
use crate::names::Names;

mod edit;
mod empty;
mod form;
mod index;
mod new;
mod single;

/// Writes all the actions views
pub(crate) fn write_crud_templates(names: &Names, fields: &[Field]) -> Result<Vec<Change>> {
    let view_mod = format!("./src/views/{}/mod.rs", &names.view_mod);
    Ok(vec![
        Change::new(view_mod, "")?.append().add_parent_mod(),
        single::write_crud_template(names, fields)?,
        index::write_crud_template(names)?,
        form::write_crud_template(names, fields)?,
        new::write_crud_template(names, fields)?,
        edit::write_crud_template(names, fields)?,
    ])
}

pub(crate) fn write_empty_templates(names: &Names, actions: &[Action]) -> Result<Vec<Change>> {
    if actions.is_empty() {
        return Ok(Vec::default());
    }
    let view_mod_name = &names.view_mod;
    let mut changes = Vec::with_capacity(actions.len() + 1);
    let path = format!("./src/views/{view_mod_name}/mod.rs");
    changes.push(Change::new(path, "")?.append().add_parent_mod());

    for action in actions {
        changes.push(empty::write_template(names, action)?);
    }
    Ok(changes)
}

/// write the `use` code that includes the model
pub(crate) fn usings(names: &Names) -> String {
    let model_mod = &names.model_mod;
    let model_struct = &names.model_struct;
    format!(
        r#"use crate::models::{model_mod}::{model_struct};
use crate::views::layouts::MainLayout;
use std::sync::Arc;
use gumbo_lib::Session;
use gumbo_lib::view::app_path;
use yew::prelude::*;
"#
    )
}
