pub mod asset_definition;
pub mod cache;
pub mod providers;

use std::path::PathBuf;

use thiserror::Error;

use crate::{
    config::{Config, ConfigError},
    warn,
};

#[derive(Error, Debug)]
pub enum AssetError {
    #[error(transparent)]
    Io(#[from] std::io::Error),
    #[error(transparent)]
    Zip(#[from] zip::result::ZipError),
    #[error("Asset {0} is not installed")]
    NotInstalled(String),
    #[error(transparent)]
    Config(#[from] ConfigError),
}

pub fn uninstall(id: String) -> Result<(), AssetError> {
    let config = Config::get()?;

    match config.get_asset_info(&id) {
        Some(asset_def) => {
            let archive = cache::local::get(&id)?;
            let paths = archive.get_archive_paths(&asset_def.include, &asset_def.exclude);

            for path in paths {
                crate::fs::safe_remove_file(&path.extract_path)?;
            }

            for path in &asset_def.include {
                walkdir::WalkDir::new(path)
                    .into_iter()
                    .filter_map(|e| e.ok())
                    .map(|e| e.path().to_path_buf())
                    .for_each(|path| {
                        if get_file_count(&path) == 0 {
                            match crate::fs::safe_remove_dir(&path) {
                                Ok(_) => (),
                                Err(e) => {
                                    warn!("Failed to remove directory {}: {}", path.display(), e)
                                }
                            }
                        }
                    });
            }

            Ok(())
        }
        None => Err(AssetError::NotInstalled(id)),
    }
}

fn get_file_count(path: &PathBuf) -> usize {
    if path.is_file() {
        1
    } else if path.is_dir() {
        let mut count = 0;
        for entry in std::fs::read_dir(path).unwrap() {
            let entry = entry.unwrap();
            let entry_path = entry.path();
            count += get_file_count(&entry_path);
        }
        count
    } else {
        0
    }
}
