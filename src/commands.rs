pub mod init;
pub mod install;

use clap::Subcommand;

#[derive(Subcommand)]
pub enum Commands {
    #[command()]
    /// Initializes your Godot project to use godam as your addon manager
    Init,
    /// Installs the specified addon to your Godot project, adding it to the godam configuration.
    #[command(alias = "i")]
    Install {
        /// The name of the asset you want to install
        #[arg(index = 1)]
        name: Option<String>,
    },
    /// Uninstalls the specified addon from your Godot project, removing it from the godam configuration.
    #[command(alias = "u")]
    Uninstall,
    /// Cleans the godam cache folder
    #[command(alias = "c")]
    Clean,
}
