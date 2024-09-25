//! godam::fs contains wrappers for all filesystem utilities used in the repository

pub const ADDONS_GITIGNORE_CONTENT: &str = "*\n!.gitignore\n!godam.toml\n.godam";

use std::{
    env::current_dir,
    fs::File,
    io::{Read, Result, Write},
    path::{Path, PathBuf},
};

pub fn safe_remove_dir(path: &Path) -> Result<()> {
    let asserted_path = get_path_asserted_within_project(path)?;
    std::fs::remove_dir_all(&asserted_path)
}

pub fn safe_create_dir(path: &Path) -> Result<()> {
    let asserted_path = get_path_asserted_within_project(path)?;
    std::fs::create_dir_all(asserted_path)
}

pub fn safe_write<C>(path: &Path, contents: C) -> Result<()>
where
    C: AsRef<[u8]>,
{
    let asserted_path = get_path_asserted_within_project(path)?;
    std::fs::write(asserted_path, contents)
}

pub fn safe_remove_file(path: &Path) -> Result<()> {
    let asserted_path = get_path_asserted_within_project(path)?;
    std::fs::remove_file(asserted_path)
}

pub fn exists(path: &std::path::Path) -> std::io::Result<bool> {
    std::fs::exists(path)
}

pub fn open(path: &Path) -> Result<File> {
    File::open(path)
}

pub fn create(path: &Path) -> Result<File> {
    std::fs::File::create(path)
}

pub fn copy<R, W>(from: &mut R, to: &mut W) -> Result<()>
where
    R: ?Sized,
    W: ?Sized,
    R: Read,
    W: Write,
{
    std::io::copy(from, to)?;
    Ok(())
}

pub fn read_string(path: &Path) -> Result<String> {
    std::fs::read_to_string(path)
}

fn get_path_asserted_within_project(path: &Path) -> Result<PathBuf> {
    let current_dir = std::path::absolute(current_dir()?)?;
    let target_path = std::path::absolute(path)?;
    assert!(target_path.starts_with(&current_dir));
    Ok(target_path)
}

pub mod path {
    use std::path::{Path, PathBuf};

    const CONFIG_PATH: &str = "./addons/godam.toml";
    const GODOT_PROJECT_FILE_PATH: &str = "./project.godot";

    const CACHE_PATH: &str = "./addons/.godam";

    const ADDONS_PATH: &str = "./addons";
    const ADDONS_ZIP_PATTERN: &str = "addons";
    const ADDONS_GITIGNORE_PATH: &str = "./addons/.gitignore";

    pub fn get_config_path() -> &'static Path {
        Path::new(CONFIG_PATH)
    }

    pub fn get_project_file_path() -> &'static Path {
        Path::new(GODOT_PROJECT_FILE_PATH)
    }

    pub fn get_cache_path() -> &'static Path {
        Path::new(CACHE_PATH)
    }

    pub fn get_cached_zip_path(id: &str) -> PathBuf {
        get_cache_path().join(id).with_extension("zip")
    }

    pub fn get_addons_path() -> &'static Path {
        Path::new(ADDONS_PATH)
    }

    pub fn get_install_folder_path(install_folder: &str) -> PathBuf {
        get_addons_path().join(install_folder)
    }

    pub fn get_gitignore_path() -> &'static Path {
        Path::new(ADDONS_GITIGNORE_PATH)
    }

    pub fn get_out_path_from_archive_path(archive_path: &str) -> Option<PathBuf> {
        archive_path
            .find(ADDONS_ZIP_PATTERN)
            .map(|start| PathBuf::new().join(&archive_path[start..]))
    }

    #[cfg(test)]
    mod tests {
        use crate::fs::get_path_asserted_within_project;

        use super::*;
        #[test]
        fn paths_are_within_working_directory() -> Result<(), Box<dyn std::error::Error>> {
            let cache_path = get_cache_path();
            let cached_zip_path = get_cached_zip_path("1234");

            let _ = get_path_asserted_within_project(cache_path)?;
            let _ = get_path_asserted_within_project(&cached_zip_path)?;

            Ok(())
        }
    }
}
