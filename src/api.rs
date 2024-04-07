use anyhow::Result;
use semver::Version;
use serde::{Deserialize, Serialize};

use crate::assets::Asset;

#[derive(Deserialize, Serialize, Clone)]
pub struct AssetResponse {
    result: Vec<Asset>,
}

pub async fn get_assets(name: &str, version: &Version) -> Result<Vec<Asset>> {
    let version_str = version.to_string();
    let request_url = format!(
        "https://godotengine.org/asset-library/api/asset?filter={name}&godot_version={version_str}"
    );
    let response = reqwest::get(&request_url).await?;

    let godot_response = response.json::<AssetResponse>().await?;
    Ok(godot_response.result)
}
