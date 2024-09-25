use thiserror::Error;

use crate::{
    api::{self, AssetSearchResult},
    godot, info,
};
#[derive(Error, Debug)]
pub enum SearchError {
    #[error(transparent)]
    Godot(#[from] godot::GodotError),
    #[error(transparent)]
    Request(#[from] api::ApiError),
}

pub async fn run(asset_name: &str) -> Result<(), SearchError> {
    let version = godot::get_project_version()?;
    let assets = api::get_assets_by_name(asset_name, &version).await?;

    for AssetSearchResult {
        title,
        asset_id,
    } in &assets
    {
        info!("{asset_id}: {title}");
    }

    Ok(())
}
