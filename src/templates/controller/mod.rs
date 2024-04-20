use super::modrs::append_module;
use super::TemplateError;
use crate::names::Names;
use crate::templates::main::append_service;
use std::fs::File;
use std::io::Write;
use std::path::Path;
use std::str::FromStr;

mod index;

#[derive(Debug, Clone, Copy)]
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
    // add the controller to the module
    let ctr_name = format!("{name}_controller");
    append_module(root_path, "./src/controllers/mod.rs", &ctr_name)?;

    // make sure the controller file exists. Adds the use as the top
    let mut ctr_path = root_path.to_path_buf();
    ctr_path.push(format!("./src/controllers/{ctr_name}.rs"));
    write_head(&ctr_path)?;

    // add an action for each action
    for action in actions {
        match action {
            Action::Index => index::write_template(name, &ctr_path)?,
            //Action::Show => show::write_template(&ctr_path)?,
            //Action::New => new::write_template(&ctr_path)?,
            //Action::Create => create::write_template(&ctr_path)?,
            //Action::Edit => edit::write_template(&ctr_path)?,
            //Action::Update => update::write_template(&ctr_path)?,
            //Action::Delete => delete::write_template(&ctr_path)?,
            _ => todo!(),
        }
    }

    Ok(())
}

/// Writes all the actions fully build wired up with views
pub(crate) fn write_crud_templates(root_path: &Path, names: &Names) -> Result<(), TemplateError> {
    let usemodel = format!(
        "use crate::models::{}::{};\n\n",
        &names.model_mod, &names.model_struct
    );
    let parts = [HEAD.to_string(), usemodel, index::crud_template(names)];

    let ctr_name = &names.controller_mod;
    append_service(root_path, format!("{ctr_name}::index"))?;

    // add the controller to the module
    let ctr_name = &names.controller_mod;
    append_module(root_path, "./src/controllers/mod.rs", ctr_name)?;

    // write the contents to the controller file
    let mut ctr_path = root_path.to_path_buf();
    ctr_path.push(&names.controller_path);

    let mut file = File::options()
        .write(true)
        .truncate(true)
        .create(true)
        .open(ctr_path)?;
    let code = parts.join("\n");
    file.write_all(code.as_bytes())?;

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

static HEAD: &str = "use crate::errors::Result;
use crate::DbClient;
use crate::helpers::render;
use welds::prelude::*;
use actix_web::{get, web::Path, HttpResponse};
";
