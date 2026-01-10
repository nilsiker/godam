use std::path::PathBuf;

use crate::asset::cache::asset_archive::AssetArchive;

pub mod asset_archive;
pub mod local;

pub trait Cache {
    fn write(
        &self,
        id: &str,
        object: CacheableObject,
    ) -> Result<CachedObject, crate::asset::AssetError>;
    fn get(&self, id: &str) -> Result<CachedObject, crate::asset::AssetError>;
    fn has(&self, id: &str) -> Result<bool, crate::asset::AssetError>;
    fn clear(&self) -> Result<(), crate::asset::AssetError>;
}

pub enum CacheableObject {
    Archive(Vec<u8>),
    Directory(PathBuf),
}

pub enum CachedObject {
    Archive(AssetArchive),
    Directory(PathBuf),
}

impl CachedObject {
    pub fn copy_to_project(
        &mut self,
        include: &[String],
        exclude: &Option<Vec<String>>,
    ) -> Result<(), crate::asset::AssetError> {
        match self {
            CachedObject::Archive(archive) => archive.extract(include, exclude),
            CachedObject::Directory(_dir_path) => unimplemented!(),
        };

        Ok(())
    }
}
