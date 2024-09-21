use anyhow::Result;
use semver::Version;
use serde::{Deserialize, Serialize};

use crate::assets::Asset;

#[derive(Deserialize, Serialize, Clone)]
pub struct AssetResponse {
    result: Asset,
}

pub async fn get_asset_by_name(name: &str, version: &Version) -> Result<Asset> {
    let version_str = version.to_string();
    let request_url = format!(
        "https://godotengine.org/asset-library/api/asset?filter={name}&godot_version={version_str}&max_results=1"
    );
    let response = reqwest::get(&request_url).await?;

    let godot_response = response.json::<AssetResponse>().await?;

    get_asset_by_id(&godot_response.result.asset_id).await
}

pub async fn get_asset_by_id(id: &str) -> Result<Asset> {
    let request_url = format!("https://godotengine.org/asset-library/api/asset/{id}");
    let asset = reqwest::get(&request_url).await?.json::<Asset>().await?;
    Ok(asset)
}
