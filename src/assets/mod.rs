pub mod cache;
pub mod consts;
pub mod error;

use cache::AssetArchive;
use error::AssetError;
use serde::{Deserialize, Serialize};

use crate::{
    config::Config,
    fs::{
        self,
        path::{get_addons_path, get_install_folder_path, get_out_path_from_archive_path},
        safe_remove_dir,
    },
};

#[derive(Deserialize, Serialize, Clone, Debug, PartialEq)]
pub struct AssetInfo {
    pub asset_id: String,
    pub title: String,
    pub download_url: String,
}

pub fn install(asset_archive: AssetArchive) -> Result<(), AssetError> {
    let (_, zip_paths_to_extract) = asset_archive.get_plugin_name_and_files_to_extract()?;

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
    Ok(())
}

pub fn get_install_folders_in_project() -> Result<Vec<String>, AssetError> {
    let addons_path = get_addons_path();

    let folders = fs::get_folders_in_directory(addons_path)?;
    Ok(folders)
}

pub fn uninstall(asset: &AssetInfo) -> Result<(), AssetError> {
    let config = Config::get()?;

    match config.get_install_folder(&asset.asset_id) {
        Some(install_folder) => {
            let asset_path = get_install_folder_path(install_folder);
            safe_remove_dir(&asset_path)?;
            Ok(())
        }
        None => Err(AssetError::NotInstalled(asset.asset_id.clone())),
    }
}
