use crate::change::Change;
use crate::errors::Result;
use crate::names::table::extract_table_name;
use crate::templates::migrations::migration_list_append;
use std::path::Path;

pub(crate) fn write_template(rootpath: &Path, name: &str) -> Result<Vec<Change>> {
    let stamp = super::timestamp();

    println!("NAMES: {name:#?}");

    let migration_name = format!("m{stamp}_{name}");

    let path = format!("./src/migrations/{migration_name}.rs");
    let code = build(name, &migration_name)?;

    Ok(vec![
        Change::new_from_path(path, code)?,
        migration_list_append(rootpath, &migration_name)?,
    ])
}

fn build(name: &str, migration_name: &str) -> Result<String> {
    let tablename = extract_table_name(name).unwrap_or(name);

    let mut parts = vec![HEAD.trim().to_owned(), fn_name()];
    parts.push(format!(
        "\n    let m = change_table(state, \"{tablename}\").unwrap().add_column(\"\", Type::Bool);"
    ));

    parts.push(";\n".to_owned());
    parts.push(fn_tail(migration_name));

    Ok(parts.join(""))
}

static HEAD: &str = r#"
use welds::errors::Result;
use welds::migrations::prelude::*;

"#;

fn fn_name() -> String {
    "\n\npub(super) fn step(state: &TableState) -> Result<MigrationStep> {".to_string()
}

fn fn_tail(migration_name: &str) -> String {
    format!("    Ok(MigrationStep::new(\"{migration_name}\", m))\n}}")
}
