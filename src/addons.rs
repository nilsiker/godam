use std::{
    io::{Read, Seek},
    path::{Path, PathBuf},
    str::FromStr,
};

use anyhow::{anyhow, Result};
use thiserror::Error;
use zip::ZipArchive;

#[derive(Error, Debug)]
pub enum AddonError {
    #[error("Could not find addon root folder.")]
    RootNotFound,
}

fn find_addon_root(dir: &Path) -> Result<PathBuf> {
    if let Some(name) = dir.file_name() {
        if name == "addons" {
            return Ok(dir.to_owned());
        }
    }

    for entry in std::fs::read_dir(dir)? {
        let entry = entry?;
        let path = entry.path();

        if path.is_dir() {
            return find_addon_root(&path);
        }
    }

    Err(anyhow!(AddonError::RootNotFound))
}

pub async fn download_addon(url: &str, output_dir: &Path) -> Result<()> {
    let resp = reqwest::get(url).await?;
    let bytes = resp.bytes().await?;
    let cursor = std::io::Cursor::new(bytes);
    let archive = zip::read::ZipArchive::new(cursor)?;

    unpack_addon(archive)?;
    Ok(())
}

fn unpack_addon<R: Read + Seek>(mut archive: ZipArchive<R>) -> Result<()> {
    for i in 0..archive.len() {
        let mut file = archive.by_index(i)?;
        println!("found archive object {}", file.name());
        let mut zip_path = Path::new(file.name()).to_path_buf();
        if !zip_path.starts_with("addons") {
            let is_dir = zip_path.to_string_lossy().ends_with("/");
            let mut comps = zip_path.components();
            comps.next();
            if comps.clone().count() == 0 {
                continue;
            }
            zip_path = comps.as_path().to_path_buf();
            if is_dir {
                let str = zip_path.to_string_lossy() + "/";
                zip_path = PathBuf::from_str(&str)?;
                println!("slash appended");
            }
            println!("after trimming: {zip_path:?}");
        }
        if !zip_path.starts_with("addons") {
            continue;
        }
        if !zip_path.exists() {
            if zip_path.to_string_lossy().ends_with("/") {
                println!("creating dir {zip_path:?}");
                std::fs::create_dir_all(zip_path)?;
            } else {
                println!("copying file {:?}", &zip_path);
                let mut out_file = std::fs::File::create(zip_path)?;
                std::io::copy(&mut file, &mut out_file)?;
            }
        }
    }

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    #[tokio::test]
    async fn test_unpack() {
        let metroidvania = "https://github.com/KoBeWi/Metroidvania-System/archive/17e1fed0ec7cd1a9bffbe58301edbab49b02b16e.zip";
        download_addon(metroidvania, Path::new("./out_dir"))
            .await
            .unwrap();
    }
}
