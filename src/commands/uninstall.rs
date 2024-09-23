use thiserror::Error;

use crate::{
    assets,
    config::{self, Config},
    console::Progress,
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

    let progress = Progress::new();

    if id == "*" {
        uninstall_all(&mut config, &progress)?;
    } else {
        uninstall_single(id, &mut config, &progress);
    }
    Ok(())
}

fn uninstall_single(id: &str, config: &mut Config, progress: &Progress) {
    let uninstalling = progress.start_single(format!("Uninstalling {id}"), Some("  "));
    let uninstalling_files = progress.start_single(format!("Removing addon folder",), Some("   "));
    let removing_from_godam =
        progress.start_single(format!("Removing from godam configuration"), Some("   "));

    let asset = match config.asset(id) {
        Ok(asset) => asset.clone(),
        Err(e) => {
            Progress::abandon_single(
                uninstalling,
                format!("Asset is not managed by godam {}: {e}", id),
            );
            return;
        }
    };

    match assets::uninstall(&asset) {
        Ok(()) => (),
        Err(e) => {
            Progress::abandon_single(
                uninstalling_files,
                format!("Warning: could not remove addon files {}: {e}", asset.title),
            );
        }
    }

    match config.remove_asset(id) {
        Ok(()) => (),
        Err(e) => {
            Progress::abandon_single(
                removing_from_godam,
                format!(
                    "Warning: could not update godam configuration {}: {e}",
                    asset.title
                ),
            );
            return;
        }
    }
    Progress::finish_single(
        uninstalling,
        format!("Successfully removed {}", asset.title),
    );
}

fn uninstall_all(config: &mut Config, progress: &Progress) -> Result<(), UninstallError> {
    for asset in config.assets.clone() {
        let id = asset.asset_id.clone();
        uninstall_single(&id, config, &progress);
    }
    Ok(())
}
