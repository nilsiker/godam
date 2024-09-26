use thiserror::Error;

use crate::config;

#[derive(Error, Debug)]
pub enum UninstallError {
    #[error(transparent)]
    Config(#[from] config::ConfigError),

    #[error("IO error: {0}")]
    Io(#[from] std::io::Error),

    #[error(transparent)]
    AssetError(#[from] crate::assets::error::AssetError),
}
