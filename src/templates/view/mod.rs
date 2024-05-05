use crate::action::Action;
use crate::fields::Field;
use crate::names::Names;
use crate::templates::modrs::append_module;
use crate::templates::TemplateError;
use std::path::Path;

mod edit;
mod empty;
mod form;
mod index;
mod new;
mod single;

/// Writes all the actions views
pub(crate) fn write_crud_templates(
    root_path: &Path,
    names: &Names,
    fields: &[Field],
) -> Result<(), TemplateError> {
    single::write_crud_template(root_path, names, fields)?;
    index::write_crud_template(root_path, names)?;
    form::write_crud_template(root_path, names, fields)?;
    new::write_crud_template(root_path, names, fields)?;
    edit::write_crud_template(root_path, names, fields)?;

    // add this view mod to the module of all views
    let view_mod = &names.view_mod;
    append_module(root_path, "./src/views/mod.rs", view_mod)?;

    Ok(())
}

pub(crate) fn write_empty_templates(
    root_path: &Path,
    names: &Names,
    actions: &[Action],
) -> Result<(), TemplateError> {
    if actions.is_empty() {
        return Ok(());
    }
    let view_mod_name = &names.view_mod;
    append_module(root_path, "./src/views/mod.rs", view_mod_name)?;
    for action in actions {
        empty::write_template(root_path, names, action)?;
    }
    Ok(())
}

/// write the `use` code that includes the model
pub(crate) fn usings(names: &Names) -> String {
    let model_mod = &names.model_mod;
    let model_struct = &names.model_struct;
    format!(
        r#"use crate::models::{model_mod}::{model_struct};
use crate::views::layouts::MainLayout;
use std::sync::Arc;
use yew::prelude::*;
"#
    )
}
