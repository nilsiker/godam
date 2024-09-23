use std::io::Cursor;

use thiserror::Error;
use tokio::task::JoinSet;
use zip::ZipArchive;

use crate::{
    api,
    assets::{install, AssetArchive, AssetInfo},
    cache,
    config::{self, Config},
    console::{Progress, Step},
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

pub async fn run(ids: &Option<Vec<String>>) -> Result<(), InstallError> {
    let mut config = Config::get()?;

    if let Some(ids) = ids {
        for id in ids {
            if config.asset(id).is_err() {
                let asset = api::get_asset_by_id(id).await?;
                config.add_asset(asset)?;
            }
        }
    }

    let mut progress = Progress::new();

    progress.start(Step::Resolve);
    let assets = match resolve_asset_info() {
        Ok(assets) => {
            progress.finish(Step::Resolve);
            assets
        }
        Err(e) => {
            progress.abandon(Step::Resolve, Box::new(e));
            return Ok(());
        }
    };

    let not_installed_assets: Vec<AssetInfo> =
        assets.into_iter().filter(|a| !a.is_installed()).collect();

    if not_installed_assets.is_empty() {
        return Ok(());
    }

    progress.start(Step::Fetch);
    let archives = match fetch_assets(not_installed_assets, &progress).await {
        Ok(archives) => {
            progress.finish(Step::Fetch);
            archives
        }
        Err(e) => {
            progress.abandon(Step::Fetch, Box::new(e));
            return Ok(());
        }
    };

    progress.start(Step::Extract);
    let successful_extracts = extract_from_archives(archives, &progress).await;
    progress.finish(Step::Extract);

    update_config(&mut config, successful_extracts)?;
    Ok(())
}

fn resolve_asset_info() -> Result<Vec<AssetInfo>, InstallError> {
    let config = Config::get()?;

    Ok(config.assets)
}

async fn fetch_assets(
    assets: Vec<AssetInfo>,
    progress: &Progress,
) -> Result<Vec<AssetArchive>, InstallError> {
    let mut tasks: JoinSet<Result<AssetArchive, InstallError>> = JoinSet::new();

    for asset in assets {
        let pb = progress.start_single(asset.title.clone(), Some("    "));

        tasks.spawn(async move {
            let archive: AssetArchive = match cache::get(&asset) {
                Ok(hit) => {
                    Progress::finish_single(pb, format!("{}: {}", asset.asset_id, asset.title));
                    hit
                }

                Err(_) => {
                    let blob = api::download(&asset).await?;
                    cache::write_to_cache(&asset.asset_id, &blob)?;
                    let cursor: Box<dyn ReadSeek> = Box::new(Cursor::new(blob.bytes));
                    Progress::finish_single(pb, format!("{}: {}", asset.asset_id, asset.title));

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

async fn extract_from_archives(
    archives: Vec<AssetArchive>,
    progress: &Progress,
) -> Vec<(String, String)> {
    let mut install_tasks = JoinSet::new();

    for archive in archives {
        let archive_id = archive.id.clone();
        let pb = progress.start_single(format!("{archive_id}.zip..."), Some("    "));
        install_tasks.spawn(async move {
            match install(archive) {
                Ok(install_folder) => {
                    Progress::finish_single(pb, format!("{archive_id}.zip"));
                    Some((archive_id, install_folder))
                }
                Err(e) => {
                    Progress::abandon_single(pb, format!("{archive_id}.zip: {e}"));
                    None
                }
            }
        });
    }

    let results = install_tasks.join_all().await;
    results.into_iter().flatten().collect()
}

fn update_config(config: &mut Config, results: Vec<(String, String)>) -> Result<(), InstallError> {
    for (id, install_folder) in results {
        config.register_install_folder(&id, install_folder)?;
    }

    Ok(())
}
