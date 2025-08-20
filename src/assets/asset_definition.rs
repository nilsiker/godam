use std::{
    io::Cursor,
    path::PathBuf,
    sync::{Arc, Mutex},
};

use indicatif::ProgressBar;
use serde::{Deserialize, Serialize};
use thiserror::Error;
use zip::{result::ZipError, ZipArchive};

use crate::{
    addons_dir::AddonsDirError,
    asset_providers::{asset_lib::AssetLib, github::GitHub, AssetProvider, AssetProviderError},
    config::{Config, ConfigError},
    console::GodamProgressMessage,
    fs::{self, path::get_install_folder_path},
    traits::ReadSeek,
};

use super::{asset_archive::AssetArchive, asset_source::AssetSource, cache, AssetError};

#[derive(Serialize, Deserialize, Clone)]
pub struct AssetDefinition {
    pub id: String,
    pub title: String,
    pub source: AssetSource,
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
    #[error(transparent)]
    AddonsDir(#[from] AddonsDirError),
    #[error(transparent)]
    Config(#[from] ConfigError),
    #[error("An error occurred when locking resources for a thread.")]
    Mutex,
}

impl AssetDefinition {
    pub async fn install(
        &self,
        progress: &ProgressBar,
        config: Arc<Mutex<Config>>,
    ) -> Result<(), AssetDefinitionError> {
        match self.source {
            AssetSource::AssetLib => self.install_asset_lib(progress, config).await,
            AssetSource::Local => self.install_local(progress, config),
            AssetSource::Git => self.install_git(progress, config).await,
        }
    }

    async fn install_asset_lib(
        &self,
        progress: &ProgressBar,
        config: Arc<Mutex<Config>>,
    ) -> Result<(), AssetDefinitionError> {
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
                    id: id.to_string(),
                    archive: ZipArchive::new(cursor)?,
                }
            }
        };

        progress.start("Unpacking", title);
        let folder_name = archive.install()?;

        let mut config = config.lock().map_err(|_| AssetDefinitionError::Mutex)?;
        config.set_install_folder(id, folder_name)?;

        Ok(())
    }

    async fn install_git(
        &self,
        progress: &ProgressBar,
        config: Arc<Mutex<Config>>,
    ) -> Result<(), AssetDefinitionError> {
        progress.start("Fetching", &self.title);
        let cache_id = self.id.replace("/", ":");
        let archive = match cache::get(&cache_id) {
            Ok(hit) => hit,
            Err(_) => {
                let blob = GitHub.download(&self.id).await?;
                cache::write_to_cache(&cache_id, &blob)?;
                let cursor: Box<dyn ReadSeek> = Box::new(Cursor::new(blob.bytes));
                AssetArchive {
                    id: self.id.clone(),
                    archive: ZipArchive::new(cursor)?,
                }
            }
        };

        progress.start("Installing", &self.title);
        let install_folder = archive.install()?;

        progress.start("Finalizing", &self.title);
        config
            .lock()
            .map_err(|_| AssetDefinitionError::Mutex)?
            .set_install_folder(&self.id, install_folder)?;

        Ok(())
    }

    fn install_local(
        &self,
        progress: &ProgressBar,
        config: Arc<Mutex<Config>>,
    ) -> Result<(), AssetDefinitionError> {
        let id = &self.id;
        let path = PathBuf::from(id);
        let folder_name = path
            .file_name()
            .expect("Path should have a file name")
            .to_str()
            .expect("File name should be valid UTF-8")
            .to_owned();

        let dest_dir = get_install_folder_path(&folder_name);

        progress.start("Installing", &self.title);

        fs::symlink::symlink_dir(path, dest_dir)?;

        config
            .lock()
            .map_err(|_| AssetDefinitionError::Mutex)?
            .set_install_folder(id, folder_name)?;

        Ok(())
    }
}
