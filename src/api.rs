use anyhow::Result;
use semver::Version;
use serde::{Deserialize, Serialize};

use crate::assets::{Asset, AssetSearchResult};


#[derive(Deserialize, Serialize, Clone)]
pub struct AssetResponse {
    result: Vec<Asset>,
}

#[derive(Deserialize, Serialize, Clone)]
pub struct AssetSearchResponse {
    result: Vec<AssetSearchResult>,
}

pub async fn get_assets_by_name(name: &str, version: &Version) -> Result<Vec<AssetSearchResult>> {
    let version_str = version.to_string();
    let request_url = format!(
        "https://godotengine.org/asset-library/api/asset?filter={name}&godot_version={version_str}"
    );
    let response = reqwest::get(&request_url).await?;

    let asset_search_response = response.json::<AssetSearchResponse>().await?;
    
    Ok(asset_search_response.result)
}

pub async fn get_asset_by_id(id: &str) -> Result<Asset> {
    let request_url = format!("https://godotengine.org/asset-library/api/asset/{id}");
    let asset = reqwest::get(&request_url).await?.json::<Asset>().await?;
    Ok(asset)
}
