use cruet::Inflector;

use super::modrs::append_module;
use super::TemplateError;
use crate::action::Action;
use crate::fields::Field;
use crate::names::Names;
use crate::templates::main::append_service;
use std::collections::HashSet;
use std::fs::File;
use std::io::Write;
use std::path::Path;

mod create;
mod edit;
mod empty;
mod index;
mod new;
mod update;

pub(crate) fn write_template(
    root_path: &Path,
    names: &Names,
    actions: &[Action],
) -> Result<(), TemplateError> {
    let ctr_name = &names.controller_mod;

    // add the controller to the module
    append_module(root_path, "./src/controllers/mod.rs", ctr_name)?;

    // make sure the controller file exists. Adds the use as the top
    let mut ctr_path = root_path.to_path_buf();
    ctr_path.push(format!("./src/controllers/{ctr_name}/mod.rs"));
    super::ensure_directory_exists(&ctr_path)?;

    let mut parts = vec![HEAD.to_string()];

    let new_methods: HashSet<_> = actions
        .iter()
        .map(|a| a.method.as_str())
        .filter(|m| m != &"get" && m != &"post")
        .collect();
    for m in new_methods {
        parts.push(format!("use actix_web::{m};"));
    }

    // add an action for each action
    for action in actions {
        parts.push("".to_owned());
        parts.push(empty::template(names, action));
        let actionname = action.name.to_snake_case();
        append_service(root_path, format!("{ctr_name}::{actionname}"))?;
    }

    let mut file = File::create(ctr_path)?;
    let code = parts.join("\n");
    file.write_all(code.as_bytes())?;

    Ok(())
}

/// Writes all the actions fully build wired up with views
pub(crate) fn write_crud_templates(
    root_path: &Path,
    names: &Names,
    fields: &[Field],
) -> Result<(), TemplateError> {
    let usemodel = format!(
        "use crate::models::{}::{};",
        &names.model_mod, &names.model_struct
    );
    let parts = [
        HEAD.to_string(),
        usemodel.to_owned(),
        "mod create_params;\nuse create_params::CreateParams;".to_owned(),
        "mod update_params;\nuse update_params::UpdateParams;".to_owned(),
        "".to_owned(),
        index::crud_template(names),
        "".to_owned(),
        new::crud_template(names),
        "".to_owned(),
        create::crud_template(names),
        "".to_owned(),
        edit::crud_template(names),
        "".to_owned(),
        update::crud_template(names),
    ];

    let ctr_name = &names.controller_mod;
    append_service(root_path, format!("{ctr_name}::index"))?;
    append_service(root_path, format!("{ctr_name}::new"))?;
    append_service(root_path, format!("{ctr_name}::create"))?;
    append_service(root_path, format!("{ctr_name}::edit"))?;
    append_service(root_path, format!("{ctr_name}::update"))?;

    // add the controller to the module
    let ctr_name = &names.controller_mod;
    append_module(root_path, "./src/controllers/mod.rs", ctr_name)?;

    // write the contents to the controller file
    let mut ctr_path = root_path.to_path_buf();
    ctr_path.push(&names.controller_path);
    super::ensure_directory_exists(&ctr_path)?;

    let mut file = File::create(ctr_path)?;
    let code = parts.join("\n");
    file.write_all(code.as_bytes())?;

    // Add the params models
    create::write_params(root_path, names, fields)?;
    update::write_params(root_path, names, fields)?;

    Ok(())
}

// A simple implementation of `% touch path` (ignores existing files)
fn write_head(path: &Path) -> std::io::Result<()> {
    if path.exists() {
        return Ok(());
    }
    let mut file = File::options()
        .write(true)
        .truncate(true)
        .create(true)
        .open(path)?;
    file.write_all(HEAD.as_bytes())?;
    Ok(())
}

static HEAD: &str = "use crate::errors::{Result, ServerError};
use crate::DbClient;
use crate::helpers::{render, redirect};
use welds::prelude::*;
use actix_web::{get, post, web::Path, web::Form, HttpResponse};
";
