use std::{fs::File, path::PathBuf, str::FromStr};

use anyhow::Result;
use thiserror::Error;
use zip::ZipArchive;

use crate::assets::Asset;

const CACHE_PATH: &str = "./addons/.godam";

#[derive(Error, Debug)]
enum CacheError {
    #[error("Asset zip was not found in cache.")]
    FileOpenFailed,
    #[error("Asset zip could not be read - file is possibly corrupted.")]
    ZipReadFailed,
}

pub fn init() -> Result<()> {
    std::fs::create_dir_all(CACHE_PATH)?;
    Ok(())
}

pub fn get(asset: &Asset) -> Result<ZipArchive<File>> {
    let file_path = PathBuf::from_str(CACHE_PATH)?
        .join(&asset.asset_id)
        .with_extension("zip");

    let file = File::open(file_path).map_err(|_| CacheError::FileOpenFailed)?;
    let archive = zip::read::ZipArchive::new(file).map_err(|_| CacheError::ZipReadFailed)?;

    Ok(archive)
}

pub async fn download_asset(asset: &Asset) -> Result<ZipArchive<File>> {
    let Asset {
        download_url,
        asset_id,
        ..
    } = asset;

    println!("Downloading {}...", asset.title);

    let resp = reqwest::get(download_url).await?;
    let bytes = resp.bytes().await?;

    let zip_path = PathBuf::from_str(CACHE_PATH)?
        .join(asset_id)
        .with_extension("zip");

    std::fs::write(zip_path, bytes)?;

    println!("Downloaded {}!", asset.title);
    get(asset)
}

pub fn clean() -> Result<()> {
    let cache_path = PathBuf::from_str(CACHE_PATH)?;
    let cache_dir = cache_path.read_dir()?;

    let mut removed = vec![];

    for entry in cache_dir {
        match entry {
            Ok(entry) => {
                std::fs::remove_file(entry.path())?;
                removed.push(entry.file_name());
            }
            Err(_) => panic!("should not happen"),
        }
    }
    println!("removed {} cached assets", removed.len());
    for path in removed {
        println!("- {}", path.into_string().expect("ok path"));
    }
    Ok(())
}

fn cache_zip_path(id: &str) -> Result<PathBuf> {
    let buf = PathBuf::from_str(CACHE_PATH)?
        .join(id)
        .with_extension("zip");
    Ok(buf)
}
