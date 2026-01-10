use std::path::{Path, PathBuf};

use zip::ZipArchive;

use crate::traits::ReadSeek;

use super::AssetError;

pub struct AssetArchive {
    pub archive: ZipArchive<Box<dyn ReadSeek>>,
}

#[derive(Debug)]
pub struct ArchivePath {
    pub archive_path: PathBuf,
    pub extract_path: PathBuf,
}

impl AssetArchive {
    /// Installs the asset archive to the addons directory.
    /// # Returns
    /// A `Result` containing the folder name of the installed asset or an `AssetError`
    pub fn install(
        self,
        include: &Vec<String>,
        exclude: &Option<Vec<String>>,
    ) -> Result<(), AssetError> {
        let archive_paths = self.get_archive_paths(include, exclude);

        let mut archive = self.archive;

        for path in &archive_paths {
            let mut contents = archive.by_path(&path.archive_path)?;

            // create parent dir if not exists
            if let Some(parent) = path.extract_path.parent() {
                if !parent.as_os_str().is_empty() && !crate::fs::exists(parent)? {
                    crate::fs::safe_create_dir(parent)?;
                }
            }

            // create file
            if !path.extract_path.exists() {
                let mut out_file = crate::fs::create(&path.extract_path)?;
                crate::fs::copy(&mut contents, &mut out_file)?;
            }
        }

        Ok(())
    }

    pub fn get_archive_paths(
        &self,
        include: &Vec<String>,
        exclude: &Option<Vec<String>>,
    ) -> Vec<ArchivePath> {
        let root_dir = self.get_root_dir();

        self.archive
            .file_names()
            .filter(|name| !name.ends_with('/'))
            .map(PathBuf::from)
            .filter(|path| {
                for include_path in include {
                    let start_path = match root_dir.clone() {
                        Some(dir) => dir.join(include_path),
                        None => path.clone(),
                    };

                    if path.starts_with(start_path) {
                        return true;
                    }
                }
                false
            })
            .filter(|path| {
                if let Some(exclude_list) = exclude {
                    for exclude_path in exclude_list {
                        let start_path = match root_dir.clone() {
                            Some(dir) => dir.join(exclude_path),
                            None => path.clone(),
                        };
                        if path.starts_with(start_path) {
                            return false;
                        }
                    }
                }
                true
            })
            .map(|path| ArchivePath {
                archive_path: path.clone(),
                extract_path: match root_dir.clone() {
                    Some(dir) => path.strip_prefix(dir).unwrap().to_path_buf(),
                    None => path.clone(),
                },
            })
            .collect::<Vec<ArchivePath>>()
    }

    fn get_root_dir(&self) -> Option<PathBuf> {
        self.archive
            .root_dir(AssetArchive::root_dir_filter)
            .unwrap()
    }

    fn root_dir_filter(_path: &Path) -> bool {
        true
    }
}
