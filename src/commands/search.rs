use thiserror::Error;

use crate::{
    godot::{
        asset_library::{get_assets_by_name, AssetSearchResult},
        error::{AssetLibraryError, GodotProjectError},
        project::get_version,
    },
    info,
};
#[derive(Error, Debug)]
pub enum SearchError {
    #[error(transparent)]
    Godot(#[from] GodotProjectError),
    #[error(transparent)]
    Request(#[from] AssetLibraryError),
}

pub async fn exec(asset_name: &str) -> Result<(), SearchError> {
    let version = get_version()?;
    let assets = get_assets_by_name(asset_name, &version).await?;

    for AssetSearchResult { title, asset_id } in &assets {
        info!("{asset_id}: {title}");
    }

    Ok(())
}
