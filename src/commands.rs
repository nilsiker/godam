use std::path::Path;

use anyhow::Result;
use clap::Subcommand;

use crate::{
    addons,
    api::get_asset,
    assets::{try_find_asset_unambiguously, Asset},
    config::Config,
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
        println!("Downloading {}", asset.title);
        let Asset { download_url, .. } = get_asset(&asset.asset_id).await?;
        match download_url {
            Some(url) => addons::download_addon(&url, Path::new(".")).await?,
            None => panic!(
                "faulty config file, missing download url for addon {}",
                asset.title
            ),
        }
    }

    Ok(())
}

pub async fn add(name: &str) -> Result<()> {
    let mut config = Config::get()?;
    let asset = try_find_asset_unambiguously(name, &config.godot_version).await?;
    config.add_asset(asset)
}

pub fn rm(name: &str) -> Result<()> {
    let mut config = Config::get()?;
    config.remove_asset(name)?;
    Ok(())
}
