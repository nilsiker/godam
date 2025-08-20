pub mod asset_archive;
pub mod asset_definition;
pub mod asset_source;
pub mod cache;
pub mod consts;
pub mod plugin_config;

use thiserror::Error;

use crate::{
    config::{Config, ConfigError},
    fs::{path::get_install_folder_path, safe_remove_dir},
};

#[derive(Error, Debug)]
pub enum AssetError {
    #[error("Invalid asset structure. No addons folder was identified for asset with id {0}")]
    InvalidAssetStructure(String),
    #[error(transparent)]
    Io(#[from] std::io::Error),
    #[error(transparent)]
    Zip(#[from] zip::result::ZipError),
    #[error("Asset {0} is not installed")]
    NotInstalled(String),
    #[error(transparent)]
    Config(#[from] ConfigError),
}

pub fn uninstall(id: String) -> Result<(), AssetError> {
    let config = Config::get()?;

    match config.get_install_folder(&id) {
        Some(install_folder) => {
            let asset_path = get_install_folder_path(install_folder);
            safe_remove_dir(&asset_path)?;
            Ok(())
        }
        None => Err(AssetError::NotInstalled(id)),
    }
}
