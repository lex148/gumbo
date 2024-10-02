use crate::action::Action;
use crate::change::Change;
use crate::errors::Result;
use crate::fields::Field;
use crate::names::Names;
use cruet::Inflector;
use std::collections::HashSet;

mod create;
mod edit;
mod empty;
mod index;
mod new;
mod update;

pub(crate) fn write_template(names: &Names, actions: &[Action]) -> Result<Vec<Change>> {
    let ctr_name = &names.controller_mod;
    let path = format!("./src/controllers/{ctr_name}/mod.rs");

    let mut parts = vec![HEAD.to_string()];

    let new_methods: HashSet<_> = actions
        .iter()
        .map(|a| a.method.as_str())
        .filter(|m| m != &"get" && m != &"post")
        .collect();
    for m in new_methods {
        parts.push(format!("use actix_web::{m};"));
    }
    // add the code for each action
    for action in actions {
        parts.push("".to_owned());
        parts.push(empty::template(names, action));
    }

    let code = parts.join("\n");
    let mut changes = vec![Change::new(path, code)?.add_parent_mod()];

    // wireup each action to actix
    for action in actions {
        let actionname = action.name.to_snake_case();
        let route = format!("{ctr_name}::{actionname}");
        changes.push(Change::append_service(route)?);
    }

    Ok(changes)
}

/// Writes all the actions fully build wired up with views
pub(crate) fn write_crud_templates(
    names: &Names,
    fields: &[Field],
) -> crate::errors::Result<Vec<Change>> {
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
    let code = parts.join("\n");
    let ctr_name = &names.controller_mod;

    Ok(vec![
        Change::new_from_path(&names.controller_path, code)?.add_parent_mod(),
        Change::append_service(format!("{ctr_name}::index"))?,
        Change::append_service(format!("{ctr_name}::new"))?,
        Change::append_service(format!("{ctr_name}::create"))?,
        Change::append_service(format!("{ctr_name}::edit"))?,
        Change::append_service(format!("{ctr_name}::update"))?,
        create::write_params(names, fields)?,
        update::write_params(names, fields)?,
    ])
}

static HEAD: &str = "use crate::errors::{Result, ServerError};
use crate::DbClient;
use gumbo_lib::view::{render, redirect};
use welds::prelude::*;
use actix_web::{get, post, web::Path, web::Form, HttpResponse};
";
