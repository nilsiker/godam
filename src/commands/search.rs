use console::style;
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

    if !assets.is_empty() {
        println!("{}", style("\nGodot Asset Library Search").underlined().bold());
    }

    for AssetSearchResult { title, asset_id } in &assets {
        println!("{}: {title}", style(asset_id).bold());
    }

    if !assets.is_empty() {
        println!(
            "\ngodam: {} assets found, try installing an asset using 'godam install <ID>'\n",
            assets.len()
        );
    } else {
        println!("godam: No results found, try a different query!\n");
    }

    Ok(())
}
