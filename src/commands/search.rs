use thiserror::Error;

use crate::{
    api::{self, AssetSearchResult},
    godot,
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

    println!(
        "\nGodam found {} {} from the Godot Asset Library:\n",
        assets.len(),
        if assets.len() > 1 {
            "results"
        } else {
            "result"
        }
    );
    for AssetSearchResult { title, asset_id } in &assets {
        println!("- ID: {asset_id} ({title})");
    }

    if !assets.is_empty() {
        println!("\nInstall an asset using [godam install <ID>]\n");
    } else {
        println!("Try a different query!\n");
    }

    Ok(())
}
