use std::{path::PathBuf, str::FromStr};

use anyhow::Result;

use crate::{
    api,
    assets::{self},
    cache,
    config::{Config, ADDONS_RELATIVE_PATH},
};

pub async fn run(id: &Option<String>) -> Result<()> {
    let mut config = Config::get()?;

    if let Some(id) = id {
        if config.asset(id).is_none() {
            let asset = api::get_asset_by_id(id).await?;
            config.add_asset(asset)?;
        }
    }

    for asset in &mut config.assets {
        if let Some(install_folder) = &asset.install_folder {
            if std::fs::exists(PathBuf::from_str(ADDONS_RELATIVE_PATH)?.join(install_folder))? {
                println!("Asset {} already installed. Skipping!", asset.title);
                continue;
            }
        }

        if cache::get(asset).is_err() {
            cache::download_asset(asset).await?;
        } else {
            println!("Found cached {}", asset.title);
        }

        let install_folder = assets::install(asset)?;
        asset.install_folder = Some(install_folder);
    }
    config.save()?;
    Ok(())
}
