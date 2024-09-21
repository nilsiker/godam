use anyhow::Result;

use crate::{
    api,
    assets::{self},
    cache,
    config::Config,
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
        if asset.install_folder.is_some() {
            println!("Asset {} already installed. Skipping!", asset.title);
            continue;
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
