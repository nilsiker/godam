use serde::{Deserialize, Serialize};
use thiserror::Error;

use crate::{
    fs::{
        exists,
        path::{get_install_folder_path, get_out_path_from_archive_path},
        safe_remove_dir,
    },
    traits::ReadSeek,
};

#[derive(Error, Debug)]
pub enum AssetError {
    #[error("Invalid asset structure. No addons folder was identified for asset with id {0}")]
    InvalidAssetStructure(String),
    #[error(transparent)]
    Io(#[from] std::io::Error),
    #[error(transparent)]
    Zip(#[from] zip::result::ZipError),
    #[error("Asset {0} is not installed")]
    NotInstalled(String),
}

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
            exists(&get_install_folder_path(install_folder)).is_ok_and(|exists| exists)
        } else {
            false
        }
    }
}

use zip::ZipArchive;

const ADDONS_PART_PATTERN: &str = "addons";

pub struct AssetArchive {
    pub id: String,
    pub archive: ZipArchive<Box<dyn ReadSeek>>,
}

impl AssetArchive {
    pub fn get_plugin_name_and_files_to_extract(
        &self,
    ) -> Result<(String, Vec<String>), AssetError> {
        let Some((plugin_name, plugin_path)) = self.get_plugin_info() else {
            return Err(AssetError::InvalidAssetStructure(self.id.to_string()));
        };

        let file_paths = self
            .archive
            .file_names()
            .filter(|file_name| file_name.starts_with(&plugin_path))
            .map(String::from)
            .collect();

        Ok((plugin_name, file_paths))
    }

    pub fn get_plugin_info(&self) -> Option<(String, String)> {
        self.archive.file_names().find_map(|file_name| {
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

pub fn install(asset_archive: AssetArchive) -> Result<String, AssetError> {
    let (plugin_name, zip_paths_to_extract) =
        asset_archive.get_plugin_name_and_files_to_extract()?;

    let mut archive = asset_archive.archive;

    for path in zip_paths_to_extract {
        let mut contents = archive.by_name(&path)?;
        let Some(out_path) = get_out_path_from_archive_path(&path) else {
            continue;
        };

        // create parent dir if not exists
        if let Some(parent) = out_path.parent() {
            if !parent.as_os_str().is_empty() && !crate::fs::exists(parent)? {
                crate::fs::safe_create_dir(parent)?;
            }
        }

        // create file
        if !out_path.exists() && !out_path.to_string_lossy().ends_with("/") {
            let mut out_file = crate::fs::create(&out_path)?;
            crate::fs::copy(&mut contents, &mut out_file)?;
        }
    }
    Ok(plugin_name)
}

pub fn uninstall(asset: &AssetInfo) -> Result<(), AssetError> {
    if !asset.is_installed() {
        return Err(AssetError::NotInstalled(asset.asset_id.clone()));
    }
    let install_folder = asset
        .install_folder
        .clone()
        .expect("install_folder is specified, ensured in code before this.");
    let asset_path = get_install_folder_path(&install_folder);
    safe_remove_dir(&asset_path)?;

    Ok(())
}
