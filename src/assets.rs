use std::{
    path::{Path, PathBuf},
    str::FromStr,
};

use anyhow::Result;
use path::installed_path;
use serde::{Deserialize, Serialize};
use thiserror::Error;

use crate::traits::ReadSeek;

#[derive(Debug, Error)]
pub enum AssetError {}

#[derive(Deserialize, Serialize, Clone, Debug, PartialEq)]
pub struct AssetInfo {
    pub asset_id: String,
    pub title: String,
    pub download_url: String,
    pub install_folder: Option<String>,
}

pub struct AssetArchive<'a> {
    pub id: &'a str,
    pub zip: zip::ZipArchive<Box<dyn ReadSeek>>,
}

impl AssetInfo {
    pub fn is_installed(&self) -> bool {
        if let Some(install_folder) = &self.install_folder {
            std::fs::exists(path::installed_path(install_folder)).is_ok_and(|exists| exists)
        } else {
            false
        }
    }
}

pub fn install(mut archive: zip::ZipArchive<Box<dyn ReadSeek>>) -> Result<String> {
    let mut plugin_folder_name: Option<String> = None;

    for i in 0..archive.len() {
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

        if !zip_path.exists() && !zip_path.to_string_lossy().ends_with("/") {
            let mut out_file = std::fs::File::create(zip_path)?;
            std::io::copy(&mut file, &mut out_file)?;
        }
    }
    Ok(plugin_folder_name.expect("plugin folder is identified"))
}

pub fn uninstall(asset: &AssetInfo) -> Result<()> {
    let install_folder = asset
        .install_folder
        .clone()
        .expect("existing install folder to be removed");
    let asset_path = installed_path(&install_folder);
    std::fs::remove_dir_all(asset_path)?;

    Ok(())
}

mod path {
    use std::{path::PathBuf, str::FromStr};

    use crate::config::ADDONS_RELATIVE_PATH;

    pub fn addons_path() -> PathBuf {
        PathBuf::from_str(ADDONS_RELATIVE_PATH).expect("valid addons relative path")
    }

    pub fn installed_path(install_folder: &str) -> PathBuf {
        addons_path().join(install_folder)
    }

    #[cfg(test)]
    mod tests {
        use super::*;
        #[test]
        fn paths_are_relative() {
            assert!(addons_path().is_relative());
            assert!(installed_path("some_asset").is_relative());
        }
    }
}
