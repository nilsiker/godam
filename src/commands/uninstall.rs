use thiserror::Error;

use crate::{
    assets,
    config::{self, Config},
};

#[derive(Error, Debug)]
pub enum UninstallError {
    #[error(transparent)]
    Config(#[from] config::ConfigError),

    #[error("IO error: {0}")]
    Io(#[from] std::io::Error),

    #[error(transparent)]
    AssetError(#[from] assets::AssetError),
}

pub async fn run(id: &str) -> Result<(), UninstallError> {
    let mut config = Config::get()?;

    if id == "*" {
        uninstall_all(&mut config)?;
    } else {
        uninstall_single(id, &mut config)?;
    }

    Ok(())
}

fn uninstall_single(id: &str, config: &mut Config) -> Result<(), UninstallError> {
    let asset = config.asset(id)?;

    match assets::uninstall(asset) {
        Ok(()) => println!("Asset {id} successfully uninstalled."),
        Err(e) => println!("Failed to uninstall asset files: {e}"),
    }

    match config.remove_asset(id) {
        Ok(()) => println!("Asset {id} successfully removed from configuration."),
        Err(e) => println!("Failed to remove asset configuration: {e}"),
    }
    Ok(())
}

fn uninstall_all(config: &mut Config) -> Result<(), UninstallError> {
    for asset in config.assets.clone() {
        let id = asset.asset_id.clone();
        uninstall_single(&id, config)?;
    }
    Ok(())
}
