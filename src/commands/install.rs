use std::io::Cursor;

use anyhow::Result;
use indicatif::{MultiProgress, ProgressBar};
use zip::ZipArchive;

use crate::{api, assets, cache, config::Config, traits::ReadSeek};

pub async fn run(id: &Option<String>) -> Result<()> {
    let mut config = Config::get()?;

    if let Some(id) = id {
        if config.asset(id).is_none() {
            let asset = api::get_asset_by_id(id).await?;
            config.add_asset(asset)?;
        }
    }

    for asset in &mut config.assets {
        if asset.is_installed() {
            println!("Asset {} already installed. Skipping!", asset.title);
            continue;
        }

        let archive: zip::ZipArchive<Box<dyn ReadSeek>> = match cache::get(asset) {
            Ok(hit) => {
                println!("Found {} in cache.", asset.title);
                hit
            }
            Err(_) => {
                println!("Downloading {}", asset.title);
                let blob = api::download(asset).await?;
                println!("Downloaded {}!", asset.title);

                println!("Caching {}", asset.title);
                cache::write_to_cache(&asset.asset_id, &blob)?;
                println!("Cached {} to {}.zip", asset.title, asset.asset_id);

                let cursor: Box<dyn ReadSeek> = Box::new(Cursor::new(blob.bytes));
                ZipArchive::new(cursor)?
            }
        };

        println!("Installing {}", asset.title);
        asset.install_folder = assets::install(archive).ok();
        println!(
            "Installed {} to {}!",
            asset.title,
            &asset.install_folder.clone().unwrap_or_default()
        );
    }

    config.save()?;

    Ok(())
}
