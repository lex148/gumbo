use crate::templates::ensure_directory_exists;
use crate::templates::TemplateError;
use std::fs::File;
use std::io::Write;
use std::path::Path;

pub(crate) fn write_template(root_path: &Path) -> Result<(), TemplateError> {
    let mut path = root_path.to_path_buf();
    path.push("./src/migrations/mod.rs");
    ensure_directory_exists(&path)?;
    let mut file = File::options()
        .write(true)
        .truncate(true)
        .create(true)
        .open(&path)?;

    file.write_all(CODE.as_bytes())?;

    Ok(())
}

static CODE: &str = r#"
use crate::errors::Result;
use welds::migrations::{create_table, types::Type, MigrationFn, MigrationStep, TableState};

pub async fn up(db: &dyn welds::TransactStart) -> Result<()> {
    let list: Vec<MigrationFn> = vec![
        /* MIGRATION LIST MARKER */
    ];
    welds::migrations::up(db, list.as_slice()).await?;
    Ok(())
}
"#;
