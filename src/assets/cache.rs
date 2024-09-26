use crate::{
    fs::{
        exists, open,
        path::{get_cache_path, get_cached_zip_path},
        safe_create_dir, safe_remove_file, safe_write,
    },
    godot::asset_library::AssetBlob,
};

use crate::{assets::AssetInfo, info, traits::ReadSeek, warn};

use zip::ZipArchive;

use super::{consts, AssetError};

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

                if first_part == consts::ADDONS_PART_PATTERN {
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

                if second_part == consts::ADDONS_PART_PATTERN {
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

pub fn write_to_cache(id: &str, archive: &AssetBlob) -> Result<(), std::io::Error> {
    ensure_cache_dir()?;

    let cached_path = get_cached_zip_path(id);
    safe_write(&cached_path, &archive.bytes)?;

    Ok(())
}

pub fn get(asset: &AssetInfo) -> Result<AssetArchive, std::io::Error> {
    ensure_cache_dir()?;

    let file_path = get_cached_zip_path(&asset.asset_id);

    let file = open(&file_path)?;
    let boxed_file: Box<dyn ReadSeek> = Box::new(file);
    let archive = zip::read::ZipArchive::new(boxed_file)?;

    Ok(AssetArchive {
        id: asset.asset_id.clone(),
        archive,
    })
}

pub fn clear() -> Result<(), std::io::Error> {
    let cache_path = get_cache_path();

    let cache_dir = cache_path.read_dir()?;

    for entry in cache_dir {
        match entry {
            Ok(entry) => {
                safe_remove_file(&entry.path())?;
                info!("Removed {} from cache", entry.file_name().to_string_lossy())
            }
            Err(e) => warn!("Failed when removing archive from cache: {e}"),
        }
    }

    Ok(())
}

fn ensure_cache_dir() -> Result<(), std::io::Error> {
    let cache_path = get_cache_path();
    if !exists(cache_path)? {
        safe_create_dir(cache_path)?;
    }
    Ok(())
}
