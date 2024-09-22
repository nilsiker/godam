//!

use std::path::PathBuf;

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

impl AssetInfo {
    pub fn is_installed(&self) -> bool {
        if let Some(install_folder) = &self.install_folder {
            std::fs::exists(path::installed_path(install_folder)).is_ok_and(|exists| exists)
        } else {
            false
        }
    }
}

use zip::ZipArchive;

const ADDONS_PATH_END: &str = "addons/";

pub struct AssetArchive(pub ZipArchive<Box<dyn ReadSeek>>);
impl AssetArchive {
    pub fn get_addons_folder_path(&self) -> Option<&str> {
        self.0
            .file_names()
            .find(|file_name| file_name.ends_with(ADDONS_PATH_END))
    }

    pub fn get_paths_under_addons(&self) -> Vec<&str> {
        match self.get_addons_folder_path() {
            Some(addons_folder) => self
                .0
                .file_names()
                .filter(|file_name| file_name.starts_with(&addons_folder))
                .collect(),
            None => vec![],
        }
    }

    pub fn get_out_path(path_under_addon: &str) -> Option<&str> {
        match path_under_addon.find("addons/") {
            Some(start) => Some(&path_under_addon[start..]),
            None => None,
        }
    }
}

pub fn install(asset_archive: AssetArchive) -> Result<String> {
    let mut install_folder = None;

    // validate zip archive, finding the addons path and then all paths to be extracted
    let zip_paths_to_extract: Vec<String> = asset_archive
        .get_paths_under_addons()
        .into_iter()
        .map(str::to_string)
        .collect();

    let mut archive = asset_archive.0;

    for path in zip_paths_to_extract {
        let mut contents = archive.by_name(&path)?;
        let out_path = match AssetArchive::get_out_path(&path) {
            Some(path) => PathBuf::new().join(path),
            None => continue,
        };

        // determine install folder
        if install_folder.is_none() {
            let comps = out_path.components();
            if comps.clone().count() == 2 {
                let comp = comps.last().expect("2 elements");
                install_folder = Some(comp.as_os_str().to_str().expect("some string").to_string());
            }
        }

        // create parent dir if not exists
        if let Some(parent) = out_path.parent() {
            if !parent.as_os_str().is_empty() && !std::fs::exists(parent)? {
                std::fs::create_dir_all(parent)?;
            }
        }

        // create file
        if !out_path.exists() && !out_path.to_string_lossy().ends_with("/") {
            let mut out_file = std::fs::File::create(out_path)?;
            std::io::copy(&mut contents, &mut out_file)?;
        }
    }
    Ok(install_folder.expect("plugin folder is identified"))
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
