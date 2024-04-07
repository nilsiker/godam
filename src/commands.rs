use anyhow::Result;
use clap::Subcommand;

use crate::{assets::service::register_addon, config::Config};

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

pub fn init() -> Result<()> {
    
    if Config::get().is_err() {
        Config::init()?;
    }

    Ok(())
}

pub fn install() -> Result<()> {
    todo!()
}

pub async fn add(name: &str) -> Result<()> {
    register_addon(name).await
}

pub fn rm(name: &str) -> Result<()> {
    todo!(r#"implement and use "{name}" to remove addon from config"#);
}
