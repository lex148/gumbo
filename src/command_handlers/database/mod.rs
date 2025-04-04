use crate::cli::DatabaseCommands;
use welds::errors::Result;
use welds::Client;

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
