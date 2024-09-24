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
    assets::{self, AssetArchive, AssetInfo},
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
}

pub async fn run(ids: &Option<Vec<String>>) -> Result<(), InstallError> {
    let mut config = Config::get()?;

    if let Some(ids) = ids {
        for id in ids {
            if config.asset(id).is_err() {
                match api::get_asset_by_id(id).await {
                    Ok(asset) => config.add_asset(asset)?,
                    Err(e) => warn!("{e}"),
                }
            }
        }
    }

    let progress = MultiProgress::new();

    let assets = Config::get()?.assets;

    let not_installed_assets: Vec<AssetInfo> =
        assets.into_iter().filter(|a| !a.is_installed()).collect();

    if not_installed_assets.is_empty() {
        return Ok(());
    }

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
    let archive: AssetArchive = match cache::get(&asset) {
        Ok(hit) => hit,

        Err(_) => {
            let blob = api::download(&asset).await?;
            cache::write_to_cache(&asset.asset_id, &blob)?;
            let cursor: Box<dyn ReadSeek> = Box::new(Cursor::new(blob.bytes));
            AssetArchive {
                id: asset.asset_id.clone(),
                archive: ZipArchive::new(cursor)?,
            }
        }
    };

    progress.running("Unpacking", &asset.title);
    let folder = match assets::install(archive).map_err(InstallError::from) {
        Ok(folder) => folder,
        Err(e) => {
            progress.failed(&asset.title, &e.to_string());
            return Err(e);
        }
    };

    progress.running("Updating", &asset.title);

    match config.lock() {
        Ok(mut config) => match config.register_install_folder(&asset.asset_id, folder) {
            Ok(()) => progress.finished("Installed", &asset.title),
            Err(e) => progress.failed(&asset.title, &e.to_string()),
        },
        Err(e) => progress.failed(&asset.title, &e.to_string()),
    }

    Ok(())
}
