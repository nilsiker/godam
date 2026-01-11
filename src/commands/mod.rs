pub mod clean;
pub mod init;
pub mod install;
pub mod list;
pub mod search;
pub mod uninstall;

use clap::Subcommand;

use crate::args::{CacheArg, SourceArg};

#[derive(Subcommand)]
pub enum Command {
    #[command()]
    /// Initializes your Godot project to use godam as your addon manager
    Init,
    /// Searches the Godot Asset Library API for assets by name.
    #[command(alias = "s")]
    Search {
        #[arg(index = 1)]
        name: String,
    },
    /// Adds the specified addon to your Godam configuration.
    #[command(alias = "i")]
    /// Installs all configured addons to your Godot project.
    Install {
        #[arg(index = 1)]
        id: Option<String>,
        #[arg(long, short = 's', default_value_t, value_enum)]
        source: SourceArg,
        #[arg(long, short = 'c', default_value_t, value_enum)]
        cache: CacheArg,
        #[arg(long, short = 'f', default_value_t = false)]
        force: bool,
        #[arg(long, short = 'i', default_values_t = ["addons".to_string()], value_delimiter = ',')]
        include: Vec<String>,
        #[arg(long, short = 'e', value_delimiter = ',')]
        exclude: Option<Vec<String>>,
    },
    /// Uninstalls the specified addon from your Godot project
    #[command(alias = "u")]
    Uninstall {
        /// The id of the asset you want to uninstall. If not specified, all assets will be uninstalled.
        #[arg(index = 1)]
        id: Option<String>,
    },
    /// Lists all assets being managed by Godam
    #[command(alias = "ls", alias = "l")]
    ///
    List,
    /// Cleans the godam cache folder
    #[command(alias = "c")]
    Clean,
}
