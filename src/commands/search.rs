use anyhow::Result;

use crate::{api, assets::AssetSearchResult, godot};

pub async fn run(asset_name: &str) -> Result<()> {
    let version = godot::get_project_version()?;
    let assets = api::get_assets_by_name(asset_name, &version).await?;

    println!("\nGodot Asset Library returned {} results:", assets.len());
    for AssetSearchResult { title, asset_id } in assets {
        println!("- {title} (ID: {asset_id})");
    }
    println!("\nInstall an asset using [godam install <ID>]\n");

    Ok(())
}
