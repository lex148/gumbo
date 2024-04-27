use crate::templates::TemplateError;
use crate::{fields::Field, names::Names};
use codegen::Scope;
use std::{fs::File, io::Write, path::Path};

pub(crate) fn crud_template(names: &Names) -> String {
    let indexaction = &names.view_mod;
    let model = &names.model_mod;
    let model_struct = &names.model_struct;
    format!(
        r#"
#[post("/{indexaction}")]
pub(crate) async fn create(db: DbClient, params: Form<CreateParams>) -> Result<HttpResponse> {{
    let mut {model} = {model_struct}::new();
    params.update(&mut {model})?;
    {model}.save(db.as_ref()).await?;
    // redirect
    redirect("/{indexaction}")
}}
"#
    )
}

pub(crate) fn write_params(
    root_path: &Path,
    names: &Names,
    fields: &[Field],
) -> Result<(), TemplateError> {
    let ctr_name = &names.controller_mod;
    let mut path = root_path.to_path_buf();
    path.push(format!("./src/controllers/{ctr_name}/create_params.rs"));
    let mut file = File::create(path)?;
    let code = build_params(names, fields);
    file.write_all(code.trim().as_bytes())?;
    Ok(())
}

pub(crate) fn build_params(names: &Names, fields: &[Field]) -> String {
    let modelmod = &names.model_mod;
    let modelstruct = &names.model_struct;

    let paramfields: Vec<_> = fields.iter().map(|f| build_param_field(names, f)).collect();
    let paramfields = paramfields.join("\n");

    let fieldsets: Vec<_> = fields.iter().map(|f| build_field_set(names, f)).collect();
    let fieldsets = fieldsets.join("\n");

    format!(
        r#"
use crate::errors::{{Result, ServerError}};
use crate::models::{modelmod}::{modelstruct};
use serde::Deserialize;

#[derive(Deserialize)]
pub(crate) struct CreateParams {{
{paramfields}
}}

impl CreateParams {{
    pub(crate) fn update(&self, {modelmod}: &mut {modelstruct}) -> Result<()> {{
{fieldsets}
        Ok(())
    }}
}}
    "#
    )
}

fn build_field_set(names: &Names, field: &Field) -> String {
    let model = &names.model_mod;
    let f = &field.name;
    let mut ty = field.ty.rust_type().to_owned();
    format!("        {model}.{f} = self.{f}.clone();")
}

fn build_param_field(names: &Names, field: &Field) -> String {
    let model = &names.model_mod;
    let f = &field.name;
    let mut ty = field.ty.rust_type().to_owned();
    if field.null {
        ty = format!("Option<{ty}>");
    }
    format!(
        r#"    #[serde(rename = "{model}[{f}]")]
    {f}: {ty},"#
    )
}
