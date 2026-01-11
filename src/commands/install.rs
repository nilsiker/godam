use std::sync::{Arc, Mutex};

use indicatif::{MultiProgress, ProgressBar};
use thiserror::Error;
use tokio::task::JoinSet;

use crate::{
    args::{CacheArg, SourceArg},
    asset::{
        self,
        asset_definition::{AssetDefinition, AssetDefinitionError},
        providers::AssetProviderError,
    },
    config::{self, Config},
    console::progress_style,
    job::install_job::{InstallArgs, InstallJob},
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
    Asset(#[from] asset::AssetError),

    #[error(transparent)]
    AssetDefinition(#[from] AssetDefinitionError),
}

pub async fn exec(
    id: &Option<String>,
    source: &SourceArg,
    cache: &CacheArg,
    force: bool,
    include: Vec<String>,
    exclude: Option<Vec<String>>,
) -> Result<(), InstallError> {
    if let Some(id) = id {
        let mut config = Config::get()?;
        // refactor this back into an "add" command
        let asset_def = AssetDefinition::new(
            id.to_string(),
            source.clone(),
            cache.clone(),
            include.clone(),
            exclude.clone(),
        )?;
        config.add_asset(id.clone(), asset_def)?;
    }

    let assets = Config::get()?.asset_definitions;
    let progress = MultiProgress::new();

    if assets.is_empty() {
        warn!("No assets are added. Try 'godam install <ID>'");
        return Ok(());
    }

    let mut tasks = JoinSet::new();

    for asset in assets.into_values() {
        let pb = progress.add(ProgressBar::new_spinner().with_style(progress_style()));

        tasks.spawn(async move {
            pb.enable_steady_tick(std::time::Duration::from_millis(100));

            let job = InstallJob::new(
                InstallArgs {
                    id: asset.id.clone(),
                    source: asset.source.clone(),
                    cache: asset.cache.clone(),
                    force,
                    include: asset.include.clone(),
                    exclude: asset.exclude.clone(),
                },
                pb,
                asset.title().cloned(),
            );

            job.run().await;
        });
    }

    tasks.join_all().await;

    Ok(())
}
