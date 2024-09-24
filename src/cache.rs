use std::fs::File;

use path::cache_zip_path;

use crate::{
    api::AssetBlob,
    assets::{AssetArchive, AssetInfo},
    info,
    traits::ReadSeek,
    warn,
};

pub fn write_to_cache(id: &str, archive: &AssetBlob) -> Result<(), std::io::Error> {
    ensure_cache_dir()?;

    let cached_path = path::cache_zip_path(id);
    std::fs::write(cached_path.clone(), &archive.bytes)?;

    Ok(())
}

pub fn get(asset: &AssetInfo) -> Result<AssetArchive, std::io::Error> {
    ensure_cache_dir()?;

    let file_path = path::cache_zip_path(&asset.asset_id);

    let file = File::open(file_path)?;
    let boxed_file: Box<dyn ReadSeek> = Box::new(file);
    let archive = zip::read::ZipArchive::new(boxed_file)?;

    Ok(AssetArchive {
        id: asset.asset_id.clone(),
        archive,
    })
}

pub fn is_zip_cached(id: &str) -> Result<bool, std::io::Error> {
    std::fs::exists(cache_zip_path(id))
}

pub fn remove_from_cache(id: &str) -> Result<(), std::io::Error> {
    let cache_path = path::cache_zip_path(id);
    std::fs::remove_file(cache_path)
}

pub fn clear() -> Result<(), std::io::Error> {
    let cache_path = path::cache_path();
    let cache_dir = cache_path.read_dir()?;

    for entry in cache_dir {
        match entry {
            Ok(entry) => {
                std::fs::remove_file(entry.path())?;
                info!("Removed {} from cache", entry.file_name().to_string_lossy())
            }
            Err(e) => warn!("Failed when removing archive from cache: {e}"),
        }
    }

    Ok(())
}

fn ensure_cache_dir() -> Result<(), std::io::Error> {
    if !std::fs::exists(path::cache_path())? {
        std::fs::create_dir_all(path::cache_path())?;
    }
    Ok(())
}

mod path {
    use std::{path::PathBuf, str::FromStr};

    const CACHE_PATH: &str = "./addons/.godam";

    pub fn cache_path() -> PathBuf {
        PathBuf::from_str(CACHE_PATH).expect("valid cache path")
    }

    pub fn cache_zip_path(id: &str) -> PathBuf {
        cache_path().join(id).with_extension("zip")
    }

    #[cfg(test)]
    mod tests {
        use super::*;
        #[test]
        fn paths_are_relative() {
            assert!(cache_path().is_relative());
            assert!(cache_zip_path("1234").is_relative());
        }
    }
}
