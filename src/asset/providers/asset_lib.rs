//! Module for querying the Godot Asset Library for assets.


use reqwest::Url;
use semver::Version;
use serde::{Deserialize, Serialize};

use crate::web_requests::{self};

use super::AssetProviderError;

#[derive(Deserialize, Serialize, Clone)]
struct AssetLibSearchResponse {
    result: Vec<AssetLibAssetMetadata>,
}

const ASSET_LIBRARY_ASSET_URL: &str = "https://godotengine.org/asset-library/api/asset";

#[derive(Deserialize, Serialize, Clone, Debug, PartialEq)]
pub struct AssetLibAssetMetadata {
    pub asset_id: String,
    pub title: String,
    pub download_url: Option<String>,
}

pub async fn query(
    name: &str,
    version: Option<&Version>,
) -> Result<Vec<AssetLibAssetMetadata>, AssetProviderError> {
    let url = match version {
        Some(version) => Url::parse_with_params(
            ASSET_LIBRARY_ASSET_URL,
            &[("filter", name), ("godot_version", &version.to_string())],
        )?,
        None => Url::parse_with_params(ASSET_LIBRARY_ASSET_URL, &[("filter", name)])?,
    };

    let asset_search_response = web_requests::get_json::<AssetLibSearchResponse>(url).await?;

    Ok(asset_search_response.result)
}

pub async fn get_asset_metadata(
    asset_id: &str,
) -> Result<AssetLibAssetMetadata, AssetProviderError> {
    let request_url = Url::parse(&format!("{ASSET_LIBRARY_ASSET_URL}/{asset_id}"))?;

    let asset = web_requests::get_json::<AssetLibAssetMetadata>(request_url).await?;

    Ok(asset)
}

pub async fn download(download_url: &str) -> Result<Vec<u8>, AssetProviderError> {
    let url = Url::parse(download_url)?;

    let bytes = web_requests::get_blob(url).await?;

    Ok(bytes)
}
