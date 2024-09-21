use anyhow::Result;

use crate::{cache, config::Config};

pub fn run() -> Result<()> {
    let config = Config::get()?;

    println!("\nGodam is managing {} assets\n", config.assets.len());
    for asset in config.assets {
        println!(
            "- {}\n\tTitle: {}\n\tInstalled: {}\n",
            asset.asset_id,
            asset.title,
            match asset.install_folder {
                Some(p) => "Yes",
                None => "No",
            }
        )
    }

    Ok(())
}
