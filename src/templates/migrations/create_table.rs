use crate::change::Change;
use crate::errors::Result;
use crate::fields::Field;
use crate::names::Names;
use crate::templates::migrations::migration_list_append;
use std::path::Path;

pub(crate) fn write_template(
    rootpath: &Path,
    names: &Names,
    fields: &[Field],
) -> Result<Vec<Change>> {
    let stamp = super::timestamp();
    let tablename = &names.table_name;

    let migration_name = if fields.is_empty() {
        format!("m{stamp}_{tablename}")
    } else {
        format!("m{stamp}_create_table_{tablename}")
    };

    let path = format!("./src/migrations/{migration_name}.rs");
    let code = build(names, fields, &migration_name)?;

    Ok(vec![
        Change::new_from_path(path, code)?,
        migration_list_append(rootpath, &migration_name)?,
    ])
}

fn build(names: &Names, fields: &[Field], migration_name: &str) -> Result<String> {
    let tablename = &names.table_name;
    let mut parts = vec![HEAD.trim().to_owned(), fn_name(names)];
    parts.push(format!("\n    let m = create_table(\"{tablename}\")"));

    let pk_field = fields.iter().find(|x| x.name == "id");
    parts.push(add_pk(pk_field));

    for f in fields {
        if f.name != "id" {
            parts.push(add_field(f));
        }
    }

    parts.push(";\n".to_owned());
    parts.push(fn_tail(migration_name));

    Ok(parts.join(""))
}

static HEAD: &str = r#"
use welds::errors::Result;
use welds::migrations::prelude::*;

"#;

fn fn_name(_name: &Names) -> String {
    "\n\npub(super) fn step(_state: &TableState) -> Result<MigrationStep> {".to_string()
}

fn add_pk(field: Option<&Field>) -> String {
    let field = match field {
        Some(f) => f,
        None => return "\n        .id(|c| c(\"id\", Type::Uuid))".to_owned(),
    };
    let ty = &field.ty;
    format!("\n        .id(|c| c(\"id\", Type::{ty}))")
}

fn add_field(field: &Field) -> String {
    let name = &field.name;
    let ty = &field.ty;
    if field.null {
        format!("\n        .column(|c| c(\"{name}\", Type::{ty}).is_null())")
    } else {
        format!("\n        .column(|c| c(\"{name}\", Type::{ty}))")
    }
}

fn fn_tail(migration_name: &str) -> String {
    format!("    Ok(MigrationStep::new(\"{migration_name}\", m))\n}}")
}

#[cfg(test)]
mod tests {

    use super::*;
    use std::str::FromStr;

    #[test]
    fn create_example_model() {
        let n = Names::new("carPrice");
        assert_eq!(n.table_name, "car_prices");
        let fields = vec![
            Field::from_str("make:string").unwrap(),
            Field::from_str("model:string:null").unwrap(),
            Field::from_str("year:int").unwrap(),
        ];

        let code = build(&n, &fields, "create_table_car_prices").unwrap();
        assert_eq!(code, EXPECTED.trim())
    }

    static EXPECTED: &str = r#"
use welds::errors::Result;
use welds::migrations::prelude::*;

pub(super) fn step(_state: &TableState) -> Result<MigrationStep> {
    let m = create_table("car_prices")
        .id(|c| c("id", Type::Int))
        .column(|c| c("make", Type::String))
        .column(|c| c("model", Type::String).null())
        .column(|c| c("year", Type::Int));
    Ok(MigrationStep::new("create_table_car_prices", m))
}
"#;
}
