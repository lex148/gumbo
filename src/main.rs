use clap::Parser;

use crate::cli::RootCommand;

pub(crate) mod cli;
pub(crate) mod command_handlers;
pub(crate) mod templates;

fn main() {
    let arg = cli::Cli::parse();

    // send the command to its command handler
    match &arg.command {
        RootCommand::Init { path } => command_handlers::init::run(path),
    }
}
