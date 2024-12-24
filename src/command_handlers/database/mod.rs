use crate::cli::DatabaseCommands;
use welds::errors::{Result, WeldsError};

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
    }
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
    let trans_start = welds::connections::connect_transstart_from_env().await?;
    let name = welds::migrations::down_last(trans_start.as_ref()).await?;
    println!("Migration Down Complete: {:?}", name);
    Ok(())
}
