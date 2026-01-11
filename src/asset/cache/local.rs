use std::path::PathBuf;

use crate::{
    asset::cache::CacheError,
    fs::{exists, path::get_cache_path, safe_create_dir, safe_remove_file},
    info, warn,
};

pub fn get(id: &str) -> Result<Option<PathBuf>, CacheError> {
    ensure_cache_dir()?;

    let cache_id = id.replace("/", "_");

    let path = get_cache_path().join(cache_id);

    if path.is_dir() {
        Ok(Some(path))
    } else {
        Ok(None)
    }
}

/// Clear the cache by removing all cached files.
pub fn clear() -> Result<(), CacheError> {
    let cache_path = get_cache_path();

    let cache_dir = cache_path.read_dir().map_err(CacheError::Io)?;

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

/// Ensure the cache directory exists, creating it if necessary.
fn ensure_cache_dir() -> Result<(), std::io::Error> {
    let cache_path = get_cache_path();
    if !exists(cache_path)? {
        safe_create_dir(cache_path)?;
    }
    Ok(())
}
