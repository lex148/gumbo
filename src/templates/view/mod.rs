use crate::fields::Field;
use crate::names::Names;
use crate::templates::modrs::append_module;
use crate::templates::TemplateError;
use std::path::Path;

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

    // add this view mod to the module of all views
    let view_mod = &names.view_mod;
    append_module(root_path, "./src/views/mod.rs", view_mod)?;

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
