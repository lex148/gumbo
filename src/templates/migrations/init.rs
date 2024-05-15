use crate::change::Change;
use crate::errors::Result;

pub(crate) fn write_template() -> Result<Vec<Change>> {
    Ok(vec![Change::new("./src/migrations/mod.rs", CODE)?])
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
