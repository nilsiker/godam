use thiserror::Error;

use crate::{
    asset_providers::{asset_lib::AssetLib, AssetProvider, AssetProviderError},
    assets::{asset_definition::AssetDefinition, asset_source::AssetSource},
    config::{Config, ConfigError},
    info, warn,
};

#[derive(Error, Debug)]
pub enum AddError {
    #[error(transparent)]
    Config(#[from] ConfigError),
    #[error(transparent)]
    AssetProvider(#[from] AssetProviderError),
    #[error("Asset was not found")]
    AssetMetadataNotFound,
}

pub async fn exec(id: &str, source: &AssetSource) -> Result<(), AddError> {
    let asset_def = match source {
        AssetSource::AssetLib => get_asset_lib_asset_def(id).await?,
        AssetSource::Local => get_local_asset_def(id)?,
        AssetSource::Git => get_git_asset_def(id).await?,
    };

    let mut config = Config::get()?;

    match config.get_asset_info(id) {
        Some(def) => warn!("{} is already added to the project. sdsd", def.title),
        None => {
            config.add_asset(id.to_string(), asset_def.clone())?;
            info!("Added {} to project.", asset_def.title);
        }
    }

    Ok(())
}

async fn get_git_asset_def(id: &str) -> Result<AssetDefinition, AddError> {
    todo!()
}

fn get_local_asset_def(id: &str) -> Result<AssetDefinition, AddError> {
    todo!()
}

async fn get_asset_lib_asset_def(id: &str) -> Result<AssetDefinition, AddError> {
    let Some(metadata) = AssetLib.lookup(id).await? else {
        return Err(AddError::AssetMetadataNotFound);
    };

    let asset_def = AssetDefinition {
        id: id.to_string(),
        title: metadata.title,
        source: AssetSource::AssetLib,
    };

    Ok(asset_def)
}
