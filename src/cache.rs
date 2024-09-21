use std::{
    fs::File,
    path::{Path, PathBuf},
    str::FromStr,
};

use anyhow::Result;
use indicatif::{ProgressBar, ProgressStyle};
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

    let resp = reqwest::get(download_url).await?;
    let bytes = resp.bytes().await?;

    let mut zip_path = PathBuf::from_str(CACHE_PATH)?.join(asset_id);

    zip_path.set_extension("zip");

    std::fs::write(zip_path, bytes)?;

    get(asset)
}

pub fn unpack_asset(asset: &Asset) -> Result<()> {
    let mut archive = get(asset)?;

    let progress = ProgressBar::new(archive.len() as u64)
        .with_style(ProgressStyle::with_template(
            "{msg} {bar:40.cyan/blue} {pos:>7}/{len:7}",
        )?)
        .with_message(format!("Installing {}: ", asset.title));

    for i in 0..archive.len() {
        progress.inc(1);

        let mut file = archive.by_index(i)?;

        let mut zip_path = Path::new(file.name()).to_path_buf();

        // Trim paths so that it begins with "addons" directory
        if !zip_path.starts_with("addons") {
            let is_dir = zip_path.to_string_lossy().ends_with("/");
            let mut comps = zip_path.components();
            comps.next();

            zip_path = comps.as_path().to_path_buf();
            if is_dir {
                let str = zip_path.to_string_lossy() + "/";
                zip_path = PathBuf::from_str(&str)?;
            }
        }

        if !zip_path.starts_with("addons") {
            continue;
        }

        if let Some(parent) = zip_path.parent() {
            if !std::fs::exists(parent)? {
                std::fs::create_dir_all(parent)?;
            }
        }

        if !zip_path.exists() {
            if !zip_path.to_string_lossy().ends_with("/") {
                let mut out_file = std::fs::File::create(zip_path)?;
                std::io::copy(&mut file, &mut out_file)?;
            }
        }
    }
    progress.finish_with_message(format!("Installed {}", asset.title));
    Ok(())
}
