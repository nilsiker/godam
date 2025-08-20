use zip::ZipArchive;

use crate::{fs::path::get_out_path_from_archive_path, traits::ReadSeek};

use super::{consts, AssetError};

pub struct AssetArchive {
    pub id: String,
    pub archive: ZipArchive<Box<dyn ReadSeek>>,
}

impl AssetArchive {
    /// Installs the asset archive to the addons directory.
    /// # Returns
    /// A `Result` containing the folder name of the installed asset or an `AssetError`
    pub fn install(self) -> Result<String, AssetError> {
        let (folder_name, zip_paths_to_extract) = self.get_plugin_name_and_files_to_extract()?;

        let mut archive = self.archive;

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

        Ok(folder_name)
    }

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

    fn get_plugin_info(&self) -> Option<(String, String)> {
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
