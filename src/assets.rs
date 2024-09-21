use anyhow::{anyhow, Result};
use serde::{Deserialize, Serialize};
use thiserror::Error;

use crate::api;

#[derive(Debug, Error)]
pub enum AssetError {
    #[error("No addon with filter {0}")]
    NoAddonsFound(String),
    #[error(
        "Could not unambiguously find addon using \"{filter}\". Candidates found: {candidates}"
    )]
    MultipleAddonsFound { filter: String, candidates: String },
}

#[derive(Deserialize, Serialize, Clone, Debug, PartialEq)]
pub struct Asset {
    pub asset_id: String,
    pub title: String,
    pub download_url: Option<String>,
}

pub async fn try_find_asset_unambiguously(
    name: &str,
    godot_version: &semver::Version,
) -> Result<Asset> {
    let assets = api::get_assets(name, godot_version).await?;

    match assets.len() {
        1 => Ok(assets[0].clone()),
        0 => Err(anyhow!(AssetError::NoAddonsFound(name.to_string()))),
        _ => Err(anyhow!(AssetError::MultipleAddonsFound {
            filter: name.to_string(),
            candidates: assets.into_iter().fold(String::new(), |mut acc, asset| {
                acc += "\n\t - ";
                acc + (&asset.title) + " (id: " + &asset.asset_id + ")"
            })
        })),
    }
}
