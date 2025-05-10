use crate::change::Change;
use crate::errors::Result;
use crate::fields::Field;
use crate::names::Names;
use std::str::FromStr;

pub(crate) fn write_template(names: &Names, fields: &[Field]) -> Result<Vec<Change>> {
    let mut fields: Vec<_> = fields.to_vec();

    // auto add a primary_key if there isn't one
    if !fields.iter().any(|x| x.primary_key) {
        fields.push(Field::from_str("id:uuid")?);
    }

    let path = &names.model_path;
    let code = build(names, &fields)?;
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
    let (id_fields, fields): (Vec<_>, Vec<_>) =
        fields.iter().partition(|f| f.primary_key || f.name == "id");

    // put the primary_key fields first
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
        col_name,
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

    // add a column rename if the column doesn't match the field
    if name != col_name {
        attrs.push(format!("#[welds(rename=\"{col_name}\")]"))
    }

    let attrs_text = attrs.join("\n");

    let mut name: String = name.to_string();
    if is_keyword(&name) {
        name = format!("r#{name}");
    }

    let ty = ty.rust_type();
    if *null {
        format!("{attrs_text}  {visibility} {name}: Option<{ty}>,")
    } else {
        format!("{attrs_text}  {visibility} {name}: {ty},")
    }
}

/// return true if the str is a rust keyword
fn is_keyword(ident: &str) -> bool {
    matches!(
        ident,
        // Strict Rust keywords
        "as" | "async" | "await" | "break" | "const" | "continue" |
        "crate" | "dyn" | "else" | "enum" | "extern" | "false" |
        "fn" | "for" | "if" | "impl" | "in" | "let" | "loop" |
        "match" | "mod" | "move" | "mut" | "pub" | "ref" |
        "return" | "self" | "Self" | "static" | "struct" | "super" |
        "trait" | "true" | "type" | "union" | "unsafe" | "use" |
        "where" | "while" |
        // Reserved / future keywords
        "abstract" | "become" | "box" | "do" | "final" |
        "macro" | "override" | "priv" | "try" | "typeof" |
        "unsized" | "virtual" | "yield"
    )
}
