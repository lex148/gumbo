use crate::cli::DatabaseCommands;
use welds::Client;
use welds::errors::Result;
mod list_tables;

/// Called to to crate a new gumbo project
pub fn run(cmd: &DatabaseCommands) {
    if let Err(err) = run_inner(cmd) {
        eprintln!("{err}");
        std::process::exit(1);
    }
}

fn run_inner(cmd: &DatabaseCommands) -> Result<()> {
    match cmd {
        DatabaseCommands::Rollback {} => welds_rollback()?,
        DatabaseCommands::TestConnection {} => test_connection()?,
        DatabaseCommands::ListTables {} => list_tables::run()?,
        DatabaseCommands::ListViews {} => list_tables::run_views()?,
        DatabaseCommands::Describe { table } => list_tables::describe(&table)?,
        DatabaseCommands::ModelFromTable { tables } => {
            println!("models from tables");
        }
    }
    Ok(())
}

fn test_connection() -> Result<()> {
    let rt = tokio::runtime::Builder::new_current_thread()
        .enable_time()
        .enable_io()
        .build()
        .unwrap();
    rt.block_on(async { test_connection_inner().await })
}

async fn test_connection_inner() -> Result<()> {
    let pool = welds::connections::connect_from_env().await?;
    pool.execute("SELECT 1", &[]).await?;
    println!("CONNECTED TO DATABASE");
    Ok(())
}

fn welds_rollback() -> Result<()> {
    let rt = tokio::runtime::Builder::new_current_thread()
        .enable_time()
        .enable_io()
        .build()
        .unwrap();
    rt.block_on(async { welds_rollback_inner().await })
}

async fn welds_rollback_inner() -> Result<()> {
    let trans_start = welds::connections::connect_from_env().await?;
    let name = welds::migrations::down_last(trans_start.as_ref()).await?;
    println!("Migration Down Complete: {:?}", name);
    Ok(())
}
