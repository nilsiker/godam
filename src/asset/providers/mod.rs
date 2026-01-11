pub mod asset_lib;
mod git;

use std::{collections::HashMap, fmt::Display, future::Future};

use serde::{Deserialize, Serialize};
use thiserror::Error;

use crate::{
    asset::{
        cache::{Cache, CacheableObject},
        AssetError,
    },
    web_requests::WebRequestError,
};

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

#[derive(Deserialize, Serialize, Clone, Debug, PartialEq)]
pub enum AssetProvider {
    AssetLib { id: String },
    Git { repo_url: String },
}

impl AssetInfo for AssetProvider {
    async fn info(&self) -> AssetMetadata {
        match self {
            AssetProvider::AssetLib { id } => {
                let mut metadata = AssetMetadata(HashMap::new());
                metadata
                    .0
                    .insert("provider".to_string(), "AssetLib".to_string());
                metadata.0.insert("id".to_string(), id.clone());
                metadata
            }
            _ => unimplemented!(),
        }
    }
}

/// Mapping of metadata key-value pairs for an asset.
#[derive(Deserialize, Serialize, Clone, Debug, PartialEq, Default)]
pub struct AssetMetadata(pub HashMap<String, String>);
impl Display for AssetMetadata {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for (key, value) in &self.0 {
            writeln!(f, "{}: {}", key, value)?;
        }
        Ok(())
    }
}

/// Mapping of asset IDs to a list of metadata key-value tuples.
pub type AssetSearchResults = HashMap<String, AssetMetadata>;

pub trait AssetSearch {
    fn search(&self, name: &str) -> Result<AssetSearchResults, AssetProviderError>;
}

pub trait AssetInfo {
    fn info(&self) -> impl std::future::Future<Output = AssetMetadata>;
}

pub trait AssetFetch {
    /// Fetches the asset to a local cache path and returns the path.
    fn fetch(&self, cache: Box<dyn Cache>) -> impl Future<Output = Result<(), AssetProviderError>>;
}

pub trait AssetInstall {
    /// Installs the asset from the local cache path to the target location.
    /// `include` is a list of file patterns to include.
    /// `exclude` is an optional list of file patterns to exclude.
    fn install(
        &self,
        cache: Box<dyn Cache>,
        include: &[String],
        exclude: &Option<Vec<String>>,
    ) -> impl Future<Output = Result<(), AssetProviderError>>;
}

impl AssetSearch for AssetProvider {
    fn search(&self, _name: &str) -> Result<AssetSearchResults, AssetProviderError> {
        match self {
            AssetProvider::AssetLib { id: _ } => {
                // Placeholder implementation
                Err(AssetProviderError::NotSupported)
            }
            _ => unimplemented!(),
        }
    }
}

impl AssetFetch for AssetProvider {
    async fn fetch(&self, cache: Box<dyn Cache>) -> Result<(), AssetProviderError> {
        match self {
            AssetProvider::AssetLib { id } => {
                let info = asset_lib::get_asset_metadata(id).await?;

                match cache.get(id) {
                    Ok(cached_object) => cached_object,
                    Err(_) => {
                        let download_url = match info.download_url {
                            Some(url) => url,
                            None => return Err(AssetProviderError::AssetNotFound),
                        };

                        let asset = asset_lib::download(&download_url).await?;

                        cache.write(id, CacheableObject::Archive(asset))?
                    }
                };

                Ok(())
            }

            _ => unimplemented!(),
        }
    }
}

impl AssetInstall for AssetProvider {
    async fn install(
        &self,
        cache: Box<dyn Cache>,
        include: &[String],
        exclude: &Option<Vec<String>>,
    ) -> Result<(), AssetProviderError> {
        match self {
            AssetProvider::AssetLib { id } => {
                let info = asset_lib::get_asset_metadata(id).await?;

                match cache.get(id) {
                    Ok(cached_object) => cached_object,
                    Err(_) => {
                        let download_url = match info.download_url {
                            Some(url) => url,
                            None => return Err(AssetProviderError::AssetNotFound),
                        };

                        let asset = asset_lib::download(&download_url).await?;

                        cache.write(id, CacheableObject::Archive(asset))?
                    }
                };
            }
            _ => unimplemented!(),
        }

        Ok(())
    }
}

pub struct AssetBlob {
    pub bytes: Vec<u8>,
}
