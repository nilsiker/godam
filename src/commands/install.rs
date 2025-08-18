use std::{
    io::Cursor,
    sync::{Arc, Mutex},
};

use indicatif::{MultiProgress, ProgressBar};
use thiserror::Error;
use tokio::task::JoinSet;
use zip::ZipArchive;

use crate::{
    addons_dir::{self, AddonsDirError},
    asset_providers::{asset_lib::AssetLib, AssetProvider, AssetProviderError},
    assets::{
        self, asset_archive::AssetArchive, asset_definition::AssetDefinition,
        asset_source::AssetSource, cache,
    },
    config::{self, Config},
    console::{progress_style, GodamProgressMessage},
    fs::path::get_install_folder_path,
    info,
    traits::ReadSeek,
    warn,
};

#[derive(Error, Debug)]
pub enum InstallError {
    #[error(transparent)]
    Config(#[from] config::ConfigError),

    #[error(transparent)]
    AssetProvider(#[from] AssetProviderError),

    #[error("Cache error: {0}")]
    Cache(#[from] std::io::Error),

    #[error(transparent)]
    Zip(#[from] zip::result::ZipError),

    #[error(transparent)]
    Asset(#[from] assets::AssetError),

    #[error("An error occured when locking resources for a thread.")]
    Mutex,

    #[error(transparent)]
    AddonsDir(#[from] AddonsDirError),
}

pub async fn exec(ids: &Option<Vec<String>>, source: &AssetSource) -> Result<(), InstallError> {
    let config = Arc::new(Mutex::new(Config::get()?));

    let progress = MultiProgress::new();

    let assets = Config::get()?.asset_definitions;

    if assets.is_empty() {
        warn!("No assets are added. Try 'godam add <ID>'");
        return Ok(());
    }

    let mut tasks = JoinSet::new();

    for (id, asset) in assets {
        let config = config.clone();
        let pb = progress.add(ProgressBar::new_spinner().with_style(progress_style()));
        tasks.spawn(async move {
            pb.enable_steady_tick(std::time::Duration::from_millis(100));

            match install_asset(&id, &asset, &pb, config).await {
                Ok(()) => pb.complete("Installed", &asset.title),
                Err(e) => pb.fail(&asset.title, &e.to_string()),
            };
        });
    }

    tasks.join_all().await;

    Ok(())
}

async fn install_asset(
    id: &str,
    asset: &AssetDefinition,
    progress: &ProgressBar,
    config: Arc<Mutex<Config>>,
) -> Result<(), InstallError> {
    progress.start("Fetching", &asset.title);
    let archive: AssetArchive = match cache::get(id) {
        Ok(hit) => hit,
        Err(_) => {
            let blob = AssetLib.download(&asset.id).await?;
            cache::write_to_cache(id, &blob)?;
            let cursor: Box<dyn ReadSeek> = Box::new(Cursor::new(blob.bytes));
            AssetArchive {
                id: id.to_string(),
                archive: ZipArchive::new(cursor)?,
            }
        }
    };

    let mut config = config.lock().map_err(|_| InstallError::Mutex)?;

    let Some((install_folder_name, _)) = archive.get_plugin_info() else {
        return Err(InstallError::Asset(
            assets::AssetError::InvalidAssetStructure("Could not find plugin name".to_string()),
        ));
    };

    if addons_dir::contains(&install_folder_name)? {
        return Ok(());
    }

    config.set_install_folder(id, install_folder_name)?;

    progress.start("Unpacking", &asset.title);
    assets::install(archive).map_err(InstallError::from)?;

    Ok(())
}
