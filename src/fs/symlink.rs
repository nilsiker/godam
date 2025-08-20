#[cfg(target_os = "linux")]
use std::path::{Path, PathBuf};

pub fn symlink_dir(src: PathBuf, dst: PathBuf) -> std::io::Result<()> {
    use std::env;

    let absolute_src = env::current_dir()?.join(src);
    let absolute_dst = env::current_dir()?.join(dst);

    #[cfg(target_os = "linux")]
    {
        std::os::unix::fs::symlink(absolute_src, absolute_dst)
    }

    #[cfg(target_os = "windows")]
    {
        std::os::windows::fs::symlink_dir(src, dst)
    }
}

pub fn symlink_exists<P: AsRef<Path>>(path: P) -> bool {
    std::fs::symlink_metadata(path).is_ok()
}
