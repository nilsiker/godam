use anyhow::Result;
use clap::Subcommand;

use crate::api::find_addon;

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
    find_addon(name).await?;
    Ok(())
}

pub fn rm(name: &str) {}
