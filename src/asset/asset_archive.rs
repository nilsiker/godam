use std::path::{Path, PathBuf};

use zip::ZipArchive;

use crate::{asset::AssetError, traits::ReadSeek};

pub struct AssetArchive {
    pub archive: ZipArchive<Box<dyn ReadSeek>>,
}

#[derive(Debug)]
pub struct ArchivePath {
    pub archive_path: PathBuf,
    pub extract_path: PathBuf,
}

impl AssetArchive {
    pub fn from_bytes(bytes: Vec<u8>) -> Result<Self, AssetError> {
        let reader: Box<dyn ReadSeek> = Box::new(std::io::Cursor::new(bytes));
        let archive = ZipArchive::new(reader)?;

        Ok(Self { archive })
    }

    /// Installs the asset archive to the addons directory.
    pub fn extract_to_cache(
        &mut self,
        id: super::cache::CacheId,
        cache_path: &Path,
    ) -> Result<PathBuf, AssetError> {
        let cache_path = cache_path.join(id.to_string());

        let archive_paths = self.get_archive_paths(&cache_path);

        let archive = &mut self.archive;

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

        Ok(cache_path)
    }

    pub fn get_archive_paths(&self, cache_path: &Path) -> Vec<ArchivePath> {
        let root_dir = self.get_root_dir();

        let paths = self
            .archive
            .file_names()
            .filter(|name| !name.ends_with('/'))
            .map(PathBuf::from)
            .map(|path| ArchivePath {
                archive_path: path.clone(),
                extract_path: match root_dir.clone() {
                    Some(dir) => {
                        let stripped_path = path.strip_prefix(dir).unwrap().to_path_buf();
                        cache_path.join(stripped_path)
                    }
                    None => cache_path.join(path),
                },
            })
            .collect::<Vec<ArchivePath>>();

        paths
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

/*
*
*
*
            .filter(|path| {
                for include_path in include {
                    let start_path = match root_dir.clone() {
                        Some(dir) => dir.join(include_path),
                        None => PathBuf::from(include_path),
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
                            None => PathBuf::from(exclude_path),
                        };
                        if path.starts_with(start_path) {
                            return false;
                        }
                    }
                }
                true
            })
*/
