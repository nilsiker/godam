use thiserror::Error;

use crate::fs::{self, path::get_install_folder_path};

#[derive(Error, Debug)]
pub enum AddonsDirError {
    #[error(transparent)]
    FileSystem(#[from] std::io::Error),
}

pub fn contains(folder: &str) -> Result<bool, AddonsDirError> {
    let path = get_install_folder_path(folder);

    Ok(fs::exists(&path)?)
}
