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

const ADDONS_PART_PATTERN: &str = "addons";

pub struct AssetArchive(pub ZipArchive<Box<dyn ReadSeek>>);
impl AssetArchive {
    pub fn get_plugin_name_and_files_to_extract(&self) -> (String, Vec<String>) {
        let (plugin_name, plugin_path) =
            self.get_plugin_info().expect("can find plugin folder path");

        let file_paths = self
            .0
            .file_names()
            .filter(|file_name| file_name.starts_with(&plugin_path))
            .map(String::from)
            .collect();

        (plugin_name, file_paths)
    }

    pub fn get_out_path(path_under_addon: &str) -> Option<&str> {
        match path_under_addon.find("addons/") {
            Some(start) => Some(&path_under_addon[start..]),
            None => None,
        }
    }

    pub fn get_plugin_info(&self) -> Option<(String, String)> {
        self.0.file_names().find_map(|file_name| {
            let mut parts = file_name.split('/');
            let mut full_path = Vec::new();

            // Check if "addons" is the first part or the second part
            if let Some(first_part) = parts.next() {
                full_path.push(first_part);

                if first_part == ADDONS_PART_PATTERN {
                    if let Some(plugin_folder) = parts.next() {
                        if !plugin_folder.is_empty() {
                            full_path.push(plugin_folder);
                            return Some((plugin_folder.to_string(), full_path.join("/")));
                        }
                    }
                }
            }

            // If not found in the first part, check for "addons" in the next layer
            if let Some(second_part) = parts.next() {
                full_path.push(second_part);

                if second_part == ADDONS_PART_PATTERN {
                    if let Some(plugin_folder) = parts.next() {
                        if !plugin_folder.is_empty() {
                            full_path.push(plugin_folder);
                            return Some((plugin_folder.to_string(), full_path.join("/")));
                        }
                    }
                }
            }

            None
        })
    }
}

pub fn install(asset_archive: AssetArchive) -> Result<String> {
    let (plugin_name, zip_paths_to_extract) = asset_archive.get_plugin_name_and_files_to_extract();

    let mut archive = asset_archive.0;

    for path in zip_paths_to_extract {
        let mut contents = archive.by_name(&path)?;
        let out_path = match AssetArchive::get_out_path(&path) {
            Some(path) => PathBuf::new().join(path),
            None => continue,
        };
        println!("{out_path:?}");

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
    Ok(plugin_name)
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
