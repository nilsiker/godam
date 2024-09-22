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

    #[error("Asset {0} does not exist in configuration.")]
    AssetNotFound(String),
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
    let asset = config
        .asset(id)
        .ok_or(UninstallError::AssetNotFound(id.to_string()))?;

    assets::uninstall(asset)?;
    config.remove_asset(id)?;
    Ok(())
}

fn uninstall_all(config: &mut Config) -> Result<(), UninstallError> {
    for asset in config.assets.clone() {
        let id = asset.asset_id.clone();
        uninstall_single(&id, config)?;
    }
    Ok(())
}
