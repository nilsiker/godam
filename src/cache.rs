use crate::fs::{
    exists, open,
    path::{get_cache_path, get_cached_zip_path},
    safe_create_dir, safe_remove_file, safe_write,
};

use crate::{
    api::AssetBlob,
    assets::{AssetArchive, AssetInfo},
    info,
    traits::ReadSeek,
    warn,
};

pub fn write_to_cache(id: &str, archive: &AssetBlob) -> Result<(), std::io::Error> {
    ensure_cache_dir()?;

    let cached_path = get_cached_zip_path(id);
    safe_write(&cached_path, &archive.bytes)?;

    Ok(())
}

pub fn get(asset: &AssetInfo) -> Result<AssetArchive, std::io::Error> {
    ensure_cache_dir()?;

    let file_path = get_cached_zip_path(&asset.asset_id);

    let file = open(&file_path)?;
    let boxed_file: Box<dyn ReadSeek> = Box::new(file);
    let archive = zip::read::ZipArchive::new(boxed_file)?;

    Ok(AssetArchive {
        id: asset.asset_id.clone(),
        archive,
    })
}

pub fn clear() -> Result<(), std::io::Error> {
    let cache_path = get_cache_path();

    let cache_dir = cache_path.read_dir()?;

    for entry in cache_dir {
        match entry {
            Ok(entry) => {
                safe_remove_file(&entry.path())?;
                info!("Removed {} from cache", entry.file_name().to_string_lossy())
            }
            Err(e) => warn!("Failed when removing archive from cache: {e}"),
        }
    }

    Ok(())
}

fn ensure_cache_dir() -> Result<(), std::io::Error> {
    let cache_path = get_cache_path();
    if !exists(cache_path)? {
        safe_create_dir(cache_path)?;
    }
    Ok(())
}
