use std::{
    path::PathBuf,
    sync::{Arc, Mutex},
};

use indicatif::{MultiProgress, ProgressBar};
use thiserror::Error;
use tokio::task::JoinSet;

use crate::{
    addons_dir::{self, AddonsDirError},
    asset_providers::{asset_lib::AssetLib, AssetProvider, AssetProviderError},
    assets::{
        self,
        asset_definition::AssetDefinition,
        asset_source::AssetSource,
        plugin_config::{PluginConfig, PluginConfigError},
    },
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

    #[error(transparent)]
    AddonsDir(#[from] AddonsDirError),

    #[error(transparent)]
    PluginConfig(#[from] PluginConfigError),
}

pub async fn exec(id: &Option<String>, source: &AssetSource) -> Result<(), InstallError> {
    if let Some(id) = id {
        let mut config = Config::get()?;

        let asset_def = match source {
            AssetSource::AssetLib => get_asset_lib_asset_def(id).await?,
            AssetSource::Local => get_local_asset_def(id)?,
            AssetSource::Git => get_git_asset_def(id).await?,
        };

        match config.get_asset_info(id) {
            Some(def) => warn!("{} is already added to the project, skipping.", def.title),
            None => {
                config.add_asset(id.to_string(), asset_def.clone())?;
                info!("Added {} to project.", asset_def.title);
            }
        }
    }

    let progress = MultiProgress::new();
    let assets = Config::get()?.asset_definitions;

    if assets.is_empty() {
        warn!("No assets are added. Try 'godam add <ID>'");
        return Ok(());
    }

    let mut tasks = JoinSet::new();

    let arc_config = Arc::new(Mutex::new(Config::get()?));
    for asset in assets.into_values() {
        let arc_config = arc_config.clone();
        let pb = progress.add(ProgressBar::new_spinner().with_style(progress_style()));

        if let Some(configured_install_folder) = Config::get()?.get_install_folder(&asset.id) {
            if addons_dir::contains(configured_install_folder)? {
                pb.complete("Already installed", &asset.title);
                continue;
            }
        }

        tasks.spawn(async move {
            pb.enable_steady_tick(std::time::Duration::from_millis(100));

            match asset.install(&pb, arc_config).await {
                Ok(_) => pb.complete("Installed", &asset.title),
                Err(e) => pb.fail(&asset.title, &e.to_string()),
            };
        });
    }

    tasks.join_all().await;

    Ok(())
}

async fn get_git_asset_def(_id: &str) -> Result<AssetDefinition, InstallError> {
    todo!()
}

fn get_local_asset_def(id: &str) -> Result<AssetDefinition, InstallError> {
    let local_path = PathBuf::from(id).join("plugin.cfg");

    let plugin_config = PluginConfig::try_from(local_path)?;

    let name = plugin_config.name();

    Ok(AssetDefinition {
        id: id.to_string(),
        title: name.to_string(),
        source: AssetSource::Local,
    })
}

async fn get_asset_lib_asset_def(id: &str) -> Result<AssetDefinition, InstallError> {
    let Some(metadata) = AssetLib.lookup(id).await? else {
        return Err(InstallError::AssetMetadataNotFound(id.to_string()));
    };

    let asset_def = AssetDefinition {
        id: id.to_string(),
        title: metadata.title,
        source: AssetSource::AssetLib,
    };

    Ok(asset_def)
}
