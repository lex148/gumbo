use crate::fields::Field;
use crate::names::Names;
use crate::templates::migrations::migration_list_append;
use crate::templates::modrs::append_module;
use crate::templates::TemplateError;
use std::fs::File;
use std::io::Write;
use std::path::Path;

pub(crate) fn write_template(
    root_path: &Path,
    names: &Names,
    fields: &[Field],
) -> Result<(), TemplateError> {
    let stamp = super::timestamp();
    let tablename = &names.table_name;
    let migration_name = format!("m{stamp}_create_table_{tablename}");

    let mut path = root_path.to_path_buf();
    path.push(format!("./src/migrations/{migration_name}.rs"));
    append_module(root_path, "./src/migrations/mod.rs", &migration_name)?;

    let mut file = File::options()
        .write(true)
        .truncate(true)
        .create(true)
        .open(&path)?;
    let code = build(names, fields)?;
    file.write_all(code.as_bytes())?;

    let modpath = format!("{migration_name}::step");
    migration_list_append(root_path, &modpath)?;

    Ok(())
}

fn build(names: &Names, fields: &[Field]) -> Result<String, TemplateError> {
    let tablename = &names.table_name;
    let mut parts = vec![HEAD.trim().to_owned(), fn_name(names)];
    parts.push(format!("\n    let m = create_table(\"{tablename}\")"));
    parts.push("\n        .id(|c| c(\"id\", Type::Int))".to_owned());

    for f in fields {
        parts.push(add_field(f));
    }

    parts.push(";\n".to_owned());
    parts.push(fn_tail(names));

    Ok(parts.join(""))
}

static HEAD: &str = r#"
use welds::errors::Result;
use welds::migrations::{create_table, types::Type, MigrationFn, MigrationStep, TableState};

"#;

fn fn_name(name: &Names) -> String {
    let name = &name.table_name;
    format!("\n\npub(super) fn step(_state: &TableState) -> Result<MigrationStep> {{")
}

fn add_field(field: &Field) -> String {
    let name = &field.name;
    let ty = &field.ty;
    if field.null {
        format!("\n        .column(|c| c(\"{name}\", Type::{ty}).null())")
    } else {
        format!("\n        .column(|c| c(\"{name}\", Type::{ty}))")
    }
}

fn fn_tail(name: &Names) -> String {
    let name = &name.table_name;
    format!("    Ok(MigrationStep::new(\"create_table_{name}\", m))\n}}")
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

        let code = build(&n, &fields).unwrap();
        assert_eq!(code, EXPECTED.trim())
    }

    static EXPECTED: &str = r#"
use welds::errors::Result;
use welds::migrations::{create_table, types::Type, MigrationFn, MigrationStep, TableState};

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
