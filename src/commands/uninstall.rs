use indicatif::{MultiProgress, ProgressBar};
use thiserror::Error;

use crate::{
    assets,
    config::{self, Config},
    console::{progress_style, GodamProgressMessage},
};

#[derive(Error, Debug)]
pub enum UninstallError {
    #[error(transparent)]
    Config(#[from] config::ConfigError),

    #[error("IO error: {0}")]
    Io(#[from] std::io::Error),

    #[error(transparent)]
    AssetError(#[from] assets::AssetError),
}

pub async fn run(id: &str) -> Result<(), UninstallError> {
    let mut config = Config::get()?;

    let progress = MultiProgress::new();

    if id == "*" {
        uninstall_all(&mut config, &progress)?;
    } else {
        uninstall_single(id, &mut config, &progress);
    }
    Ok(())
}

fn uninstall_single(id: &str, config: &mut Config, progress: &MultiProgress) {
    let pb = progress.add(ProgressBar::new_spinner().with_style(progress_style()));

    let asset = match config.asset(id) {
        Ok(asset) => asset.clone(),
        Err(e) => {
            pb.failed(id, &e.to_string());
            return;
        }
    };

    pb.running("Uninstalling", &asset.title);
    match assets::uninstall(&asset) {
        Ok(()) => (),
        Err(e) => {
            pb.failed(id, &e.to_string());
        }
    }
    pb.running("Removing", &asset.title);
    match config.remove_asset(id) {
        Ok(()) => (),
        Err(e) => {
            pb.failed(&asset.title, &e.to_string());
            return;
        }
    }
    pb.finished("Removed", &asset.title);
}

fn uninstall_all(config: &mut Config, progress: &MultiProgress) -> Result<(), UninstallError> {
    for asset in config.assets.clone() {
        let id = asset.asset_id.clone();
        uninstall_single(&id, config, progress);
    }
    Ok(())
}
