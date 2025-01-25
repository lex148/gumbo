use crate::change::Change;
use crate::errors::Result;
use crate::{fields::Field, names::Names};

pub(crate) fn crud_template(names: &Names) -> String {
    let indexaction = &names.view_mod;
    let model = &names.model_mod;
    let model_struct = &names.model_struct;
    format!(
        r#"
#[post("/{indexaction}/{{id}}")]
pub(crate) async fn update(db: DbClient, path: Path<Uuid>, params: Form<UpdateParams>, _session: Option<Session>) -> Result<HttpResponse> {{
    let id = path.into_inner();
    let mut {model} = {model_struct}::find_by_id(db.as_ref(), id)
        .await?
        .ok_or(ServerError::ResourceNotFound)?;
    params.update(&mut {model})?;
    {model}.save(db.as_ref()).await?;
    // redirect
    redirect("/{indexaction}")
}}
"#
    )
}

pub(crate) fn write_params(names: &Names, fields: &[Field]) -> Result<Change> {
    let ctr_name = &names.controller_mod;
    let path = format!("./src/controllers/{ctr_name}/update_params.rs");
    let code = build_params(names, fields);
    Ok(Change::new(path, code)?.add_parent_mod())
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
pub(crate) struct UpdateParams {{
{paramfields}
}}

impl UpdateParams {{
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
