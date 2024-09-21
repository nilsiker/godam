use anyhow::Result;

use crate::{assets, config::Config};

pub async fn run(id: &str) -> Result<()> {
    let mut config = Config::get()?;

    match config.asset(id) {
        Some(asset) => {
            assets::uninstall(asset)?;
            println!(
                "Asset {} ({}) uninstall successfully, removing folder {}",
                asset.title,
                asset.asset_id,
                asset.install_folder.clone().expect("install folder exists")
            );
        }
        None => println!("Asset with ID {id} not found in configuration."),
    }
    config.remove_asset(id)?;
    Ok(())
}
