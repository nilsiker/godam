use anyhow::Result;

use crate::config::Config;

pub fn run() -> Result<()> {
    let config = Config::get()?;

    println!("\nGodam is managing {} assets\n", config.assets.len());
    for asset in config.assets {
        println!(
            "- {}\n\tTitle: {}\n\tInstalled: {}\n",
            asset.asset_id,
            asset.title,
            match asset.install_folder {
                Some(_) => "Yes",
                None => "No",
            }
        )
    }

    Ok(())
}
