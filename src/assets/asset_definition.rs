use std::io::Cursor;

use indicatif::ProgressBar;
use serde::{Deserialize, Serialize};
use thiserror::Error;
use zip::{result::ZipError, ZipArchive};

use crate::{
    asset_providers::{asset_lib::AssetLib, github::GitHub, AssetProvider, AssetProviderError},
    console::GodamProgressMessage,
    traits::ReadSeek,
};

use super::{asset_archive::AssetArchive, asset_source::AssetSource, cache, AssetError};

#[derive(Serialize, Deserialize, Clone, Default, Debug)]
pub struct AssetDefinition {
    pub id: String,
    pub title: String,
    pub source: AssetSource,
    pub include: Vec<String>,
    pub exclude: Option<Vec<String>>,
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
        match self.source {
            AssetSource::AssetLib => self.install_asset_lib(progress).await,
            AssetSource::Github => self.install_git(progress).await,
        }
    }

    async fn install_asset_lib(&self, progress: &ProgressBar) -> Result<(), AssetDefinitionError> {
        let id = &self.id;
        let title = &self.title;

        progress.start("Fetching", title);

        let archive = match cache::get(id) {
            Ok(hit) => hit,
            Err(_) => {
                let blob = AssetLib.download(id).await?;
                cache::write_to_cache(id, &blob)?;
                let cursor: Box<dyn ReadSeek> = Box::new(Cursor::new(blob.bytes));
                AssetArchive {
                    archive: ZipArchive::new(cursor)?,
                }
            }
        };

        progress.start("Unpacking", title);
        archive.install(&self.include, &self.exclude)?;

        Ok(())
    }

    async fn install_git(&self, progress: &ProgressBar) -> Result<(), AssetDefinitionError> {
        progress.start("Fetching", &self.title);
        let cache_id = self.id.replace("/", "_");

        let archive = match cache::get(&cache_id) {
            Ok(hit) => hit,
            Err(_) => {
                let blob = GitHub.download(&self.id).await?;
                cache::write_to_cache(&cache_id, &blob)?;
                let cursor: Box<dyn ReadSeek> = Box::new(Cursor::new(blob.bytes));

                AssetArchive {
                    archive: ZipArchive::new(cursor)?,
                }
            }
        };

        progress.start("Installing", &self.title);
        archive.install(&self.include, &self.exclude)?;

        Ok(())
    }
}
