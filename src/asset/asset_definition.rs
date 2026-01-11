use std::collections::HashMap;

use serde::{Deserialize, Serialize};
use thiserror::Error;
use zip::result::ZipError;

use crate::{
    args::{CacheArg, SourceArg},
    asset::providers::AssetProviderError,
};

use super::AssetError;

#[derive(Serialize, Deserialize, Clone, Default, Debug)]
pub struct AssetDefinition {
    pub id: String,
    pub source: SourceArg,
    pub cache: CacheArg,
    pub include: Vec<String>,
    pub exclude: Option<Vec<String>>,
    pub metadata: HashMap<String, String>,
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
            metadata: HashMap::default(),
        })
    }

    pub fn title(&self) -> Option<&String> {
        self.metadata.get("title")
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

impl std::fmt::Display for AssetDefinition {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self.metadata.get("title") {
            Some(value) => write!(f, "{}", value),
            None => write!(f, "{}", self.id),
        }
    }
}
