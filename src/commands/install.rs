use std::{
    io::Cursor,
    sync::{Arc, Mutex},
};

use indicatif::{MultiProgress, ProgressBar};
use thiserror::Error;
use tokio::task::JoinSet;
use zip::ZipArchive;

use crate::{
    api,
    assets::{self, get_install_folders_in_project, AssetArchive, AssetInfo},
    cache,
    config::{self, Config},
    console::{progress_style, GodamProgressMessage},
    traits::ReadSeek,
    warn,
};

#[derive(Error, Debug)]
pub enum InstallError {
    #[error(transparent)]
    Config(#[from] config::ConfigError),

    #[error(transparent)]
    Request(#[from] api::ApiError),

    #[error("Cache error: {0}")]
    Cache(#[from] std::io::Error),

    #[error(transparent)]
    Zip(#[from] zip::result::ZipError),

    #[error(transparent)]
    Asset(#[from] assets::AssetError),
    #[error("An error occured when locking resources for a thread.")]
    Mutex,
}

pub async fn run(ids: &Option<Vec<String>>) -> Result<(), InstallError> {
    let mut config = Config::get()?;

    if let Some(ids) = ids {
        for id in ids {
            if config.get_asset(id).is_err() {
                match api::get_asset_by_id(id).await {
                    Ok(asset) => config.add_asset(asset)?,
                    Err(e) => warn!("{e}"),
                }
            }
        }
    }

    let progress = MultiProgress::new();

    let assets = Config::get()?.assets;
    let install_folders = get_install_folders_in_project()?;

    let not_installed_assets: Vec<AssetInfo> = assets
        .into_iter()
        .filter_map(|asset| {
            let Some(folder) = config.get_install_folder(&asset.asset_id) else {
                return Some(asset);
            };
            if install_folders.contains(folder) {
                None
            } else {
                Some(asset)
            }
        })
        .collect();

    let config = Arc::new(Mutex::new(config));

    let mut tasks = JoinSet::new();

    for asset in not_installed_assets {
        let config = config.clone();
        let pb = progress.add(ProgressBar::new_spinner().with_style(progress_style()));
        tasks.spawn(async move {
            pb.enable_steady_tick(std::time::Duration::from_millis(100));
            match install_asset(&asset, &pb, config).await {
                Ok(()) => pb.finished("Installed", &asset.title),
                Err(e) => pb.failed(&asset.title, &e.to_string()),
            };
        });
    }

    tasks.join_all().await;

    Ok(())
}

async fn install_asset(
    asset: &AssetInfo,
    progress: &ProgressBar,
    config: Arc<Mutex<Config>>,
) -> Result<(), InstallError> {
    progress.running("Fetching", &asset.title);
    let archive: AssetArchive = match cache::get(asset) {
        Ok(hit) => hit,

        Err(_) => {
            let blob = api::download(asset).await?;
            cache::write_to_cache(&asset.asset_id, &blob)?;
            let cursor: Box<dyn ReadSeek> = Box::new(Cursor::new(blob.bytes));
            AssetArchive {
                id: asset.asset_id.clone(),
                archive: ZipArchive::new(cursor)?,
            }
        }
    };

    // register install folder before installing
    match config.lock() {
        Ok(mut config) => {
            let install_folder_name = match archive.get_plugin_info() {
                Some((name, _)) => name,
                None => {
                    return Err(InstallError::Asset(
                        assets::AssetError::InvalidAssetStructure(
                            "Could not find plugin name".to_string(),
                        ),
                    ))
                }
            };
            if let Err(e) = config.set_install_folder(&asset.asset_id, install_folder_name) {
                progress.failed(&asset.title, &e.to_string());
            }
        }
        Err(e) => {
            progress.failed(&asset.title, &e.to_string());
            return Err(InstallError::Mutex);
        }
    }

    progress.running("Unpacking", &asset.title);
    let folder = match assets::install(archive).map_err(InstallError::from) {
        Ok(folder) => folder,
        Err(e) => {
            progress.failed(&asset.title, &e.to_string());
            return Err(e);
        }
    };

    progress.finished("Installed", &asset.title);

    Ok(())
}
