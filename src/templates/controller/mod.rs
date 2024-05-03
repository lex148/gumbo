use super::modrs::append_module;
use super::TemplateError;
use crate::fields::Field;
use crate::names::Names;
use crate::templates::main::append_service;
use std::fs::File;
use std::io::Write;
use std::path::Path;
use std::str::FromStr;

mod create;
mod edit;
mod index;
mod new;
mod update;

#[derive(Debug, Clone, Copy, PartialEq)]
pub(crate) enum Action {
    Index,
    Show,
    Edit,
    Update,
    New,
    Create,
    Delete,
}

impl FromStr for Action {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(match s.to_lowercase().as_str() {
            "index" => Action::Index,
            "show" => Action::Show,
            "edit" => Action::Edit,
            "update" => Action::Update,
            "new" => Action::New,
            "create" => Action::Create,
            "delete" => Action::Delete,
            _ => return Err(()),
        })
    }
}

pub(crate) fn write_template(
    root_path: &Path,
    name: &str,
    actions: &[Action],
) -> Result<(), TemplateError> {
    let names = Names::new(name);
    let ctr_name = &names.controller_mod;

    // add the controller to the module
    append_module(root_path, "./src/controllers/mod.rs", ctr_name)?;

    // make sure the controller file exists. Adds the use as the top
    let mut ctr_path = root_path.to_path_buf();
    ctr_path.push(format!("./src/controllers/{ctr_name}/mod.rs"));
    super::ensure_directory_exists(&ctr_path)?;

    let mut parts = vec![HEAD.to_string()];

    // add an action for each action
    for action in actions {
        if action == &Action::Index {
            parts.push("".to_owned());
            parts.push(index::crud_template(&names));
        }
        if action == &Action::New {
            parts.push("".to_owned());
            parts.push(new::crud_template(&names));
        }
        if action == &Action::Create {
            parts.push("".to_owned());
            parts.push("mod create_params;\nuse create_params::CreateParams;".to_owned());
            parts.push("".to_owned());
            parts.push(create::crud_template(&names));
            create::write_params(root_path, &names, &[])?;
        }
        if action == &Action::Edit {
            parts.push("".to_owned());
            parts.push(edit::crud_template(&names));
        }
        if action == &Action::Update {
            parts.push("".to_owned());
            parts.push("mod update_params;\nuse update_params::UpdateParams;".to_owned());
            parts.push("".to_owned());
            parts.push(update::crud_template(&names));
            update::write_params(root_path, &names, &[])?;
        }
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
