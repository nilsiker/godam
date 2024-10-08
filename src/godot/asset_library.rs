//! Handles all calls to the web

use reqwest::Url;
use semver::Version;
use serde::{Deserialize, Serialize};
use thiserror::Error;

use crate::assets::AssetInfo;

#[derive(Error, Debug)]
pub enum AssetLibraryError {
    #[error("API request failed: {0}")]
    Unhandled(#[from] reqwest::Error),
    #[error("Expected a valid ID (integer), found '{0}'")]
    InvalidId(String),
    #[error("Could not parse url")]
    ParseUrl,
}

#[derive(Deserialize, Serialize, Clone)]
pub struct AssetResponse {
    result: Vec<AssetInfo>,
}

#[derive(Deserialize, Serialize, Clone)]
pub struct AssetSearchResponse {
    result: Vec<AssetSearchResult>,
}

#[derive(Deserialize, Serialize, Clone, Debug, PartialEq)]
pub struct AssetSearchResult {
    pub asset_id: String,
    pub title: String,
}

pub struct AssetBlob {
    pub bytes: Vec<u8>,
}

pub async fn get_assets_by_name(
    name: &str,
    version: &Version,
) -> Result<Vec<AssetSearchResult>, AssetLibraryError> {
    let version_str = version.to_string();

    let url = Url::parse_with_params(
        "https://godotengine.org/asset-library/api/asset",
        &[("filter", name), ("godot_version", &version_str)],
    )
    .map_err(|_| AssetLibraryError::ParseUrl)?;

    let response = reqwest::get(url).await?;

    let asset_search_response = response.json::<AssetSearchResponse>().await?;

    Ok(asset_search_response.result)
}

pub async fn get_asset_by_id(id: &str) -> Result<AssetInfo, AssetLibraryError> {
    // TODO should validate with param type for ID
    if id.parse::<usize>().is_err() {
        return Err(AssetLibraryError::InvalidId(id.to_string()));
    }

    let request_url = format!("https://godotengine.org/asset-library/api/asset/{id}");
    let asset = reqwest::get(&request_url)
        .await?
        .json::<AssetInfo>()
        .await?;
    Ok(asset)
}

pub async fn download(asset: &AssetInfo) -> Result<AssetBlob, AssetLibraryError> {
    let resp = reqwest::get(&asset.download_url).await?;

    let bytes = resp.bytes().await?;

    Ok(AssetBlob {
        bytes: bytes.to_vec(),
    })
}
