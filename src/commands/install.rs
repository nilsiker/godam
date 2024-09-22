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
        if config.asset(id).is_none() {
            let asset = api::get_asset_by_id(id).await?;
            config.add_asset(asset)?;
        }
    }

    let assets = resolve_asset_info()?;

    let not_installed_assets = assets.into_iter().filter(|a| !a.is_installed()).collect();

    let archives = fetch_assets(not_installed_assets).await?;

    let install_results = install_from_archives(archives).await;

    update_config(&mut config, install_results)?;
    Ok(())
}

async fn install_from_archives(
    archives: Vec<AssetArchive>,
) -> Vec<(String, Result<String, std::io::Error>)> {
    let mut install_tasks = JoinSet::new();

    for archive in archives {
        install_tasks.spawn(async move {
            let archive_id = archive.id.clone();
            let install_folder = install(archive);

            (archive_id, install_folder)
        });
    }

    install_tasks.join_all().await
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

fn update_config(
    config: &mut Config,
    results: Vec<(String, Result<String, std::io::Error>)>,
) -> Result<(), std::io::Error> {
    for (id, result) in results {
        match result {
            Ok(install_folder) => config.register_install_folder(&id, install_folder),
            Err(e) => {
                println!("Error when processing asset: {e}")
            }
        }
    }

    Ok(())
}
