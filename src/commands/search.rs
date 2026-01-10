use thiserror::Error;

use crate::{
    asset::providers::{asset_lib, AssetProviderError},
    godot::project::{get_version, GodotProjectError},
    info,
};
#[derive(Error, Debug)]
pub enum SearchError {
    #[error(transparent)]
    Godot(#[from] GodotProjectError),
    #[error(transparent)]
    AssetProvider(#[from] AssetProviderError),
}

pub async fn exec(asset_name: &str) -> Result<(), SearchError> {
    let version = get_version()?;
    let assets = asset_lib::query(asset_name, Some(&version)).await?;

    assets.iter().for_each(|a| {
        info!("{}: {}", a.asset_id, a.title);
    });

    Ok(())
}
