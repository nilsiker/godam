use indicatif::{MultiProgress, ProgressBar};
use thiserror::Error;
use tokio::task::JoinSet;

use crate::{
    asset_providers::{asset_lib::AssetLib, github::GitHub, AssetProvider, AssetProviderError},
    assets::{self, asset_definition::AssetDefinition, asset_source::AssetSource},
    config::{self, Config},
    console::{progress_style, GodamProgressMessage},
    info, warn,
};

#[derive(Error, Debug)]
pub enum InstallError {
    #[error("Could not find asset metadata for ID: {0}")]
    AssetMetadataNotFound(String),
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
}

pub async fn exec(
    id: &Option<String>,
    source: &AssetSource,
    include: Vec<String>,
    exclude: Option<Vec<String>>,
) -> Result<(), InstallError> {
    if let Some(id) = id {
        let mut config = Config::get()?;

        let asset_def = match source {
            AssetSource::AssetLib => get_asset_lib_asset_def(id, include, exclude).await?,
            AssetSource::Github => get_git_asset_def(id, include, exclude).await?,
        };

        if config.get_asset_info(id).is_none() {
            config.add_asset(id.to_string(), asset_def.clone())?;
            info!("Added {} to project.", asset_def.title);
        }
    }

    let progress = MultiProgress::new();
    let assets = Config::get()?.asset_definitions;

    if assets.is_empty() {
        warn!("No assets are added. Try 'godam add <ID>'");
        return Ok(());
    }

    let mut tasks = JoinSet::new();

    for asset in assets.into_values() {
        let pb = progress.add(ProgressBar::new_spinner().with_style(progress_style()));

        tasks.spawn(async move {
            pb.enable_steady_tick(std::time::Duration::from_millis(100));

            match asset.install(&pb).await {
                Ok(_) => pb.complete("Installed", &asset.title),
                Err(e) => pb.fail(&asset.title, &e.to_string()),
            };
        });
    }

    tasks.join_all().await;

    Ok(())
}

async fn get_git_asset_def(
    id: &str,
    include: Vec<String>,
    exclude: Option<Vec<String>>,
) -> Result<AssetDefinition, InstallError> {
    let Some(metadata) = GitHub.lookup(id).await? else {
        return Err(InstallError::AssetMetadataNotFound(id.to_string()));
    };

    Ok(AssetDefinition {
        id: id.to_string(),
        title: metadata.title,
        source: AssetSource::Github,
        include,
        exclude,
    })
}

async fn get_asset_lib_asset_def(
    id: &str,
    include: Vec<String>,
    exclude: Option<Vec<String>>,
) -> Result<AssetDefinition, InstallError> {
    let Some(metadata) = AssetLib.lookup(id).await? else {
        return Err(InstallError::AssetMetadataNotFound(id.to_string()));
    };

    let asset_def = AssetDefinition {
        id: id.to_string(),
        title: metadata.title,
        source: AssetSource::AssetLib,
        include,
        exclude,
    };

    Ok(asset_def)
}
