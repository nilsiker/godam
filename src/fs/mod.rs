//! godam::fs contains wrappers for all filesystem utilities used in the repository

pub mod path;

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
