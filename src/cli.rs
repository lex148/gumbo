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

    /// Used to generate code
    #[clap(name = "generate", alias = "g")]
    Generate {
        #[clap(subcommand)]
        sub_cmd: GenerateCommands,
    },
}

#[derive(Subcommand, Debug)]
pub(crate) enum GenerateCommands {
    #[clap(name = "scaffold", alias = "s")]
    /// Scaffold out a full model/view/controller
    Scaffold {
        /// Name of the resource to scaffold. Example: `gumbo generate scaffold car make:string model:string year:int` would
        /// create a model/view/controller/migration for cars. Everything all wired up for the
        /// three fields
        name: String,
        /// List of fields for model. Example: name:string description:text:option
        ///    examples of common types: (bool, int_small, int, int_big, string, text, float, big_float, binary, uuid)
        fields: Vec<String>,
    },

    #[clap(name = "controller", alias = "c")]
    /// Generate a controller and its actions
    Controller {
        /// Name of the controller. "Cars" would generate the cars_controller
        name: String,
        /// List of actions the controller will respond to (index show new create edit update delete)
        actions: Vec<String>,
    },

    #[clap(name = "env")]
    /// Generate a .env file.
    /// Setups an .env file will all the setting needed to boot up your gumbo app.
    Env {},
}
