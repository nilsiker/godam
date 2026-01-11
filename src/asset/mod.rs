pub mod asset_archive;
pub mod asset_definition;
pub mod cache;
pub mod providers;

use std::path::PathBuf;

use thiserror::Error;

use crate::config::{Config, ConfigError};

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

    unimplemented!()
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
