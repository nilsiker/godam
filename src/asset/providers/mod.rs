pub mod asset_lib;
mod git;

use thiserror::Error;

use crate::{asset::AssetError, web_requests::WebRequestError};

#[derive(Error, Debug)]
pub enum AssetProviderError {
    #[error("Asset was not found.")]
    AssetNotFound,
    #[error(transparent)]
    WebRequest(#[from] WebRequestError),
    #[error(transparent)]
    Parse(#[from] url::ParseError),
    #[error(transparent)]
    FetchGit(#[from] git2::Error),
    #[error(transparent)]
    Io(#[from] std::io::Error),
    #[error("Unsupported operation.")]
    NotSupported,
    #[error(transparent)]
    ZipError(#[from] zip::result::ZipError),
    #[error(transparent)]
    Asset(#[from] AssetError),
}

pub struct AssetBlob {
    pub bytes: Vec<u8>,
}
