use std::{
    io::{Read, Seek},
    path::{Path, PathBuf},
    str::FromStr,
};

use anyhow::Result;
use indicatif::{ProgressBar, ProgressStyle};
use thiserror::Error;
use zip::ZipArchive;

#[derive(Error, Debug)]
pub enum AddonError {
    #[error("Could not find addon root folder.")]
    RootNotFound,
}

pub async fn download_addon(addon_name: &str, url: &str) -> Result<()> {
    let resp = reqwest::get(url).await?;
    let bytes = resp.bytes().await?;
    let cursor = std::io::Cursor::new(bytes);
    let archive = zip::read::ZipArchive::new(cursor)?;

    unpack_addon(addon_name, archive)?;
    Ok(())
}

fn unpack_addon<R: Read + Seek>(addon_name: &str, mut archive: ZipArchive<R>) -> Result<()> {
    let progress = ProgressBar::new(archive.len() as u64)
        .with_style(ProgressStyle::with_template(
            "{msg} {bar:40.cyan/blue} {pos:>7}/{len:7}",
        )?)
        .with_message(format!("Installing {addon_name}: "));

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
    progress.finish_with_message(format!("Installed {addon_name}"));
    Ok(())
}

