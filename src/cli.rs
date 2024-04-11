use clap::{Parser, Subcommand};
use std::path::PathBuf;

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
#[command(propagate_version = true)]
pub(crate) struct Cli {
    #[command(subcommand)]
    pub command: RootCommand,
}

#[derive(Subcommand, Debug)]
pub(crate) enum RootCommand {
    /// Create a new project all setup for gumbo
    Init {
        /// Path to where to initialize the new project
        path: PathBuf,
    },
}
