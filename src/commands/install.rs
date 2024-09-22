use std::io::Cursor;

use thiserror::Error;
use tokio::task::JoinSet;
use zip::ZipArchive;

use crate::{
    api,
    assets::{install, AssetArchive, AssetInfo},
    cache,
    config::{self, Config},
    traits::ReadSeek,
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
}

pub async fn run(id: &Option<String>) -> Result<(), InstallError> {
    let mut config = Config::get()?;
    if let Some(id) = id {
        if config.asset(id).is_err() {
            let asset = api::get_asset_by_id(id).await?;
            config.add_asset(asset)?;
        }
    }

    let assets = resolve_asset_info()?;

    let not_installed_assets = assets.into_iter().filter(|a| !a.is_installed()).collect();

    let archives = fetch_assets(not_installed_assets).await?;

    let successful_installs = install_from_archives(archives).await;

    update_config(&mut config, successful_installs)?;
    Ok(())
}

async fn install_from_archives(archives: Vec<AssetArchive>) -> Vec<(String, String)> {
    let mut install_tasks = JoinSet::new();

    for archive in archives {
        install_tasks.spawn(async move {
            let archive_id = archive.id.clone();
            match install(archive) {
                Ok(install_folder) => Some((archive_id, install_folder)),
                Err(e) => {
                    println!("Error while installing asset {archive_id}: {e}");
                    None
                }
            }
        });
    }

    let results = install_tasks.join_all().await;
    results.into_iter().flatten().collect()
}

fn resolve_asset_info() -> Result<Vec<AssetInfo>, InstallError> {
    let config = Config::get()?;

    println!("Resolved asset information");
    Ok(config.assets)
}

async fn fetch_assets(assets: Vec<AssetInfo>) -> Result<Vec<AssetArchive>, InstallError> {
    let mut tasks: JoinSet<Result<AssetArchive, InstallError>> = JoinSet::new();

    for asset in assets {
        tasks.spawn(async move {
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

            Ok(archive)
        });
    }

    let results = tasks.join_all().await;
    let archives = results
        .into_iter()
        .filter_map(|result| match result {
            Ok(archive) => Some(archive),
            Err(e) => {
                println!("Failed to fetch archive: {e}");
                None
            }
        })
        .collect();

    Ok(archives)
}

fn update_config(config: &mut Config, results: Vec<(String, String)>) -> Result<(), InstallError> {
    for (id, install_folder) in results {
        config.register_install_folder(&id, install_folder)?;
    }

    Ok(())
}
