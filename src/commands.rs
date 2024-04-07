use anyhow::Result;
use clap::Subcommand;

use crate::{
    api::get_asset,
    assets::{service::register_addon, Asset},
    config::Config,
    git,
};

#[derive(Subcommand)]
pub enum Commands {
    /// Initializes your Godot project to use godam as your addon manager
    Init,
    /// Installs addons specified in the godam.toml
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
        println!("Project initialized. Next, add assets using godam add <name>");
    } else {
        println!("Project already initialized. Try adding assets using godam add <name>");
    }

    Ok(())
}

pub async fn install() -> Result<()> {
    let config = Config::get()?;

    for asset in config.assets {
        let Asset {
            browse_url,
            download_commit,
            ..
        } = get_asset(&asset.asset_id).await?;

        let (Some(url), Some(commit)) = (browse_url, download_commit) else {
            println!("[{}] No url returned. Skipping...", asset.title);
            continue;
        };

        git::clone(&asset.title, &url, &commit)?;
        println!("Successfully cloned {}", asset.title);
    }

    Ok(())
}

pub async fn add(name: &str) -> Result<()> {
    register_addon(name).await
}

pub fn rm(name: &str) -> Result<()> {
    let mut config = Config::get()?;
    config.remove_asset(name)?;
    Ok(())
}
