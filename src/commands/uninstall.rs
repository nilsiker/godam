use indicatif::{MultiProgress, ProgressBar};

use crate::{
    assets,
    config::Config,
    console::{progress_style, GodamProgressMessage},
    error::UninstallError,
    warn,
};

pub fn exec(id: &Option<String>) -> Result<(), UninstallError> {
    let mut config = Config::get()?;

    let progress = MultiProgress::new();

    match id {
        Some(some_id) => uninstall_single(some_id, &mut config, &progress),
        None => {
            warn!("Do you want to uninstall all addons? ('y' to confirm)");
            let confirm = console::Term::stdout().read_char()?;
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
