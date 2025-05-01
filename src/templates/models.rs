use crate::change::Change;
use crate::errors::Result;
use crate::fields::Field;
use crate::names::Names;
use std::str::FromStr;

pub(crate) fn write_template(names: &Names, fields: &[Field]) -> Result<Vec<Change>> {
    let path = &names.model_path;
    let code = build(names, fields)?;
    Ok(vec![Change::new_from_path(path, code)?.add_parent_mod()])
}

pub(crate) fn build(names: &Names, fields: &[Field]) -> Result<String> {
    let struct_code = build_struct(names, fields)?;
    let table_attribute = names.welds_table_attribute();

    let head = format!(
        r#"use welds::prelude::*;

#[derive(Debug, WeldsModel, PartialEq)]
{table_attribute}
"#
    );

    Ok(format!("{head}{struct_code}"))
}

pub(crate) fn build_struct(names: &Names, fields: &[Field]) -> Result<String> {
    let (mut id_fields, fields): (Vec<_>, Vec<_>) = fields.iter().partition(|f| f.primary_key);
    let default_id_field: Field = Field::from_str("id:uuid")?;

    // the ID field, or use a default
    if id_fields.is_empty() {
        id_fields.push(&default_id_field);
    }

    let innerds: Vec<String> = id_fields
        .iter()
        .chain(fields.iter())
        .map(|&f| field_line(f))
        .collect();

    let innerds: String = innerds.join("\n");
    let model_name = &names.model_struct;

    let head = format!("pub(crate) struct {model_name} {{\n");

    Ok(format!("{head}  {innerds}\n}}"))
}

/// write each fields for the model
fn field_line(field: &Field) -> String {
    let Field {
        visibility,
        primary_key,
        welds_ignored,
        name,
        ty,
        null,
    } = field;

    let mut attrs: Vec<String> = Vec::default();
    if *primary_key {
        attrs.push("#[welds(primary_key)]".to_string())
    }
    if *welds_ignored {
        attrs.push("#[welds(ignore)]".to_string())
    }
    let attrs_text = attrs.join("\n");

    let ty = ty.rust_type();
    if *null {
        format!("{attrs_text}  {visibility} {name}: Option<{ty}>,")
    } else {
        format!("{attrs_text}  {visibility} {name}: {ty},")
    }
}
