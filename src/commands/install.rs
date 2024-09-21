use anyhow::Result;

use crate::{api, assets::is_asset_installed, cache, config::Config};

pub async fn run(name: &Option<String>) -> Result<()> {
    let mut config = Config::get()?;

    if let Some(asset_name) = name {
        let asset = api::get_asset_by_name(asset_name, &config.godot_version).await?;

        config.add_asset(asset)?;
        
        todo!("implement adding of addon {asset_name} before installing all");
    }

    for asset in config.assets {
        if is_asset_installed(&asset) {
            continue;
        }
        cache::download_asset(&asset).await?;
        cache::unpack_asset(&asset)?;
    }

    Ok(())
}
