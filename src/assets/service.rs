use anyhow::{anyhow, Result};

use crate::{api, assets::AssetError, config::Config};

use super::Asset;

pub async fn register_addon(name: &str) -> Result<()> {
    let mut config = Config::get()?;

    let asset = try_find_asset_unambiguously(name, &config.godot_version).await?;

    config.add_asset(asset)
}

async fn try_find_asset_unambiguously(
    name: &str,
    godot_version: &semver::Version,
) -> Result<Asset> {
    let assets = api::get_assets(name, &godot_version).await?;

    match assets.len() {
        1 => Ok(assets[0].clone()),
        0 => Err(anyhow!(AssetError::NoAddonsFound(name.to_string()))),
        _ => Err(anyhow!(AssetError::MultipleAddonsFound {
            filter: name.to_string(),
            candidates: assets
                .into_iter()
                .map(|asset| format!("\n\t - {}", asset.title))
                .collect()
        })),
    }
}
