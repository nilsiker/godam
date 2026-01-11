use indicatif::ProgressBar;
use serde::{Deserialize, Serialize};
use thiserror::Error;
use zip::result::ZipError;

use crate::{
    args::{CacheArg, SourceArg},
    asset::{
        cache::{local::LocalCache, Cache},
        providers::{AssetInstall, AssetMetadata, AssetProvider, AssetProviderError},
    },
    warn,
};

use super::AssetError;

#[derive(Serialize, Deserialize, Clone, Default, Debug)]
pub struct AssetDefinition {
    pub id: String,
    pub source: SourceArg,
    pub cache: CacheArg,
    pub include: Vec<String>,
    pub exclude: Option<Vec<String>>,
    metadata: AssetMetadata,
}
impl AssetDefinition {
    pub fn new(
        id: String,
        source: SourceArg,
        cache: CacheArg,
        include: Vec<String>,
        exclude: Option<Vec<String>>,
    ) -> Result<Self, AssetDefinitionError> {
        Ok(Self {
            id,
            source,
            cache,
            include,
            exclude,
            metadata: AssetMetadata::default(),
        })
    }
}

#[derive(Error, Debug)]
pub enum AssetDefinitionError {
    #[error(transparent)]
    AssetProvider(#[from] AssetProviderError),
    #[error(transparent)]
    Cache(#[from] std::io::Error),
    #[error(transparent)]
    Zip(#[from] ZipError),
    #[error(transparent)]
    Asset(#[from] AssetError),
}

impl AssetDefinition {
    pub async fn install(&self, progress: &ProgressBar) -> Result<(), AssetDefinitionError> {
        let provider = match self.source {
            SourceArg::AssetLib => AssetProvider::AssetLib {
                id: self.id.clone(),
            },
            SourceArg::Git => AssetProvider::Git {
                repo_url: self.id.clone(),
            },
            _ => {
                return Err(AssetDefinitionError::AssetProvider(
                    AssetProviderError::NotSupported,
                ));
            }
        };

        let cache: Box<dyn Cache> = match self.cache {
            CacheArg::Local => Box::new(LocalCache),
            CacheArg::Global => unimplemented!(),
        };

        progress.set_message("Installing");

        provider
            .install(cache, &self.include, &self.exclude)
            .await?;

        Ok(())
    }

    pub fn is_installed(&self) -> Result<bool, AssetDefinitionError> {
        warn!("is_installed is not implemented yet.");
        Ok(false)
    }
}
impl std::fmt::Display for AssetDefinition {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self.metadata.0.get("title") {
            Some(value) => write!(f, "{}", value),
            None => write!(f, "{}", self.id),
        }
    }
}
