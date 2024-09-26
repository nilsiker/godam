use indicatif::{MultiProgress, ProgressBar};
use thiserror::Error;

use crate::{
    assets::{self, AssetError},
    config::{Config, ConfigError},
    console::{progress_style, GodamProgressMessage},
    prompt_char,
};

#[derive(Error, Debug)]
pub enum UninstallError {
    #[error(transparent)]
    Config(#[from] ConfigError),

    #[error("IO error: {0}")]
    Io(#[from] std::io::Error),

    #[error(transparent)]
    AssetError(#[from] AssetError),
}

pub fn exec(id: &Option<String>) -> Result<(), UninstallError> {
    let mut config = Config::get()?;

    let progress = MultiProgress::new();

    match id {
        Some(some_id) => uninstall_single(some_id, &mut config, &progress),
        None => {
            let confirm = prompt_char!("Do you want to uninstall all addons? ('y' to confirm)");
            if confirm == 'y' {
                uninstall_all(&mut config, &progress)?;
            }
        }
    }
    Ok(())
}

fn uninstall_single(id: &str, config: &mut Config, progress: &MultiProgress) {
    let pb = progress.add(ProgressBar::new_spinner().with_style(progress_style()));

    let asset = match config.get_asset(id) {
        Ok(a) => a.clone(),
        Err(e) => {
            pb.fail(id, &e.to_string());
            return;
        }
    };

    pb.start("Uninstalling", &asset.title);
    match assets::uninstall(&asset) {
        Ok(()) => (),
        Err(e) => {
            pb.fail(id, &e.to_string());
        }
    }
    pb.start("Removing", &asset.title);
    match config.remove_asset(id) {
        Ok(()) => (),
        Err(e) => {
            pb.fail(&asset.title, &e.to_string());
            return;
        }
    }
    pb.complete("Removed", &asset.title);
}

fn uninstall_all(config: &mut Config, progress: &MultiProgress) -> Result<(), UninstallError> {
    for asset in config.assets.clone() {
        let id = asset.asset_id.clone();
        uninstall_single(&id, config, progress);
    }
    Ok(())
}
