use thiserror::Error;

use crate::config::ConfigError;

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
