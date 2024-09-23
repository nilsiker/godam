
use crate::config::{Config, ConfigError};

pub fn run() -> Result<(), ConfigError> {
    let config = Config::get()?;

    println!("godam: found {} assets in configuration:\n", config.assets.len());
    for asset in config.assets {
        println!(
            "    - {}\n\tTitle: {}\n\tInstall folder: {}\n",
            asset.asset_id,
            asset.title,
            asset.install_folder.unwrap_or("not installed".to_string())
        )
    }

    Ok(())
}
