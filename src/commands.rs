use anyhow::Result;
use clap::Subcommand;

use crate::assets::service::register_addon;

#[derive(Subcommand)]
pub enum Commands {
    /// Initializes your Godot project to use gaddon as your addon manager
    Init,
    /// Installs addons specified in the gaddons.toml
    #[command()]
    Install,
    /// Adds and installs the specified addon to your Godot project
    Add {
        name: String,
    },
    Rm {
        name: String,
    },
}

pub fn init() {}

pub fn install() {}

pub async fn add(name: &str) -> Result<()> {
    register_addon(name).await
}

pub fn rm(_name: &str) {}