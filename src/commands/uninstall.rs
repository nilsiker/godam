use anyhow::Result;

use crate::{assets, config::Config};

pub async fn run(id: &str) -> Result<()> {
    let mut config = Config::get()?;

    match id {
        "*" => uninstall_all(&mut config),
        _ => uninstall_single(id, &mut config),
    }
}

fn uninstall_single(id: &str, config: &mut Config) -> Result<()> {
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

fn uninstall_all(config: &mut Config) -> Result<()> {
    for asset in config.assets.clone() {
        let id = asset.asset_id.clone();
        uninstall_single(&id, config)?;
    }
    Ok(())
}
