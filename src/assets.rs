use std::{
    path::{Path, PathBuf},
    str::FromStr,
};

use anyhow::Result;
use indicatif::{ProgressBar, ProgressStyle};
use serde::{Deserialize, Serialize};
use thiserror::Error;

use crate::{
    cache::get,
    config::{Config, ADDONS_RELATIVE_PATH},
};

#[derive(Debug, Error)]
pub enum AssetError {}

#[derive(Deserialize, Serialize, Clone, Debug, PartialEq)]
pub struct AssetSearchResult {
    pub asset_id: String,
    pub title: String,
}

#[derive(Deserialize, Serialize, Clone, Debug, PartialEq)]
pub struct Asset {
    pub asset_id: String,
    pub title: String,
    pub download_url: String,
    pub install_folder: Option<String>,
}

pub fn install(asset: &Asset) -> Result<String> {
    let config = Config::get()?;

    let asset = config
        .asset(&asset.asset_id)
        .expect("id should exist in config");

    let mut archive = get(asset)?;

    let progress = ProgressBar::new(archive.len() as u64)
        .with_style(ProgressStyle::with_template(
            "{msg} {bar:40.cyan/blue} {pos:>7}/{len:7}",
        )?)
        .with_message(format!("Installing {}: ", asset.title));

    let mut plugin_folder_name: Option<String> = None;

    for i in 0..archive.len() {
        progress.inc(1);

        let mut file = archive.by_index(i)?;

        let mut zip_path = Path::new(file.name()).to_path_buf();

        // Trim paths so that it begins with "addons" directory
        if !zip_path.starts_with("addons") {
            let is_dir = zip_path.to_string_lossy().ends_with("/");
            let mut comps = zip_path.components();
            comps.next();

            zip_path = comps.as_path().to_path_buf();
            if is_dir {
                let str = zip_path.to_string_lossy() + "/";
                zip_path = PathBuf::from_str(&str)?;
            }
        }

        if !zip_path.starts_with("addons") {
            continue;
        }

        // Extract the plugin folder name (first directory inside "addons")
        if plugin_folder_name.is_none() {
            let relative_path = zip_path.strip_prefix("addons")?;
            if let Some(first_component) = relative_path.components().next() {
                if let Some(os_str) = first_component.as_os_str().to_str() {
                    plugin_folder_name = Some(os_str.to_string());
                }
            }
        }

        if let Some(parent) = zip_path.parent() {
            if !std::fs::exists(parent)? {
                std::fs::create_dir_all(parent)?;
            }
        }

        if !zip_path.exists() {
            if !zip_path.to_string_lossy().ends_with("/") {
                let mut out_file = std::fs::File::create(zip_path)?;
                std::io::copy(&mut file, &mut out_file)?;
            }
        }
    }
    progress.finish_with_message(format!("Installed {}", asset.title));
    Ok(plugin_folder_name.expect("plugin folder is identified"))
}

pub fn uninstall(asset: &Asset) -> Result<()> {
    let install_folder = asset
        .install_folder
        .clone()
        .expect("existing install folder to be removed");
    let asset_path = PathBuf::from_str(ADDONS_RELATIVE_PATH)?.join(install_folder);
    std::fs::remove_dir_all(asset_path)?;

    Ok(())
}
