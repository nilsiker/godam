use crate::{
    asset::cache::{asset_archive::AssetArchive, Cache, CacheableObject, CachedObject},
    fs::{
        self, exists, open,
        path::{get_cache_path, get_cached_zip_path},
        safe_create_dir, safe_remove_file,
    },
    info,
    traits::ReadSeek,
    warn,
};

pub struct LocalCache;
impl Cache for LocalCache {
    fn write(
        &self,
        id: &str,
        object: CacheableObject,
    ) -> Result<CachedObject, crate::asset::AssetError> {
        ensure_cache_dir()?;

        let cached_path = get_cached_zip_path(id);

        match object {
            CacheableObject::Archive(bytes) => {
                fs::safe_write(&cached_path, &bytes)?;
            }
            CacheableObject::Directory(_) => unimplemented!(),
        }

        let cached = self.get(id)?;

        Ok(cached)
    }

    fn get(&self, id: &str) -> Result<super::CachedObject, crate::asset::AssetError> {
        ensure_cache_dir()?;

        let file_path = get_cached_zip_path(id.replace("/", "_").as_str());

        let file = open(&file_path)?;
        let boxed_file: Box<dyn ReadSeek> = Box::new(file);
        let archive = zip::read::ZipArchive::new(boxed_file)?;

        Ok(AssetArchive { archive })
    }

    fn has(&self, id: &str) -> Result<bool, crate::asset::AssetError> {
        todo!()
    }

    fn clear(&self) -> Result<(), crate::asset::AssetError> {
        todo!()
    }
}

// Gets an archive from the cache.
pub fn get(id: &str) -> Result<AssetArchive, std::io::Error> {}

/// Clear the cache by removing all cached files.
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

/// Ensure the cache directory exists, creating it if necessary.
fn ensure_cache_dir() -> Result<(), std::io::Error> {
    let cache_path = get_cache_path();
    if !exists(cache_path)? {
        safe_create_dir(cache_path)?;
    }
    Ok(())
}
