use indicatif::{MultiProgress, ProgressBar};
use thiserror::Error;
use tokio::task::JoinSet;

use crate::{
    args::SourceArg,
    asset::{
        self,
        asset_definition::{AssetDefinition, AssetDefinitionError},
        providers::{AssetInfo, AssetProvider, AssetProviderError},
    },
    config::{self, Config},
    console::{progress_style, GodamProgressMessage},
    info, warn,
};

#[derive(Error, Debug)]
pub enum InstallError {
    #[error(transparent)]
    Config(#[from] config::ConfigError),

    #[error(transparent)]
    AssetProvider(#[from] AssetProviderError),

    #[error("Cache error: {0}")]
    Cache(#[from] std::io::Error),

    #[error(transparent)]
    Zip(#[from] zip::result::ZipError),

    #[error(transparent)]
    Asset(#[from] asset::AssetError),

    #[error(transparent)]
    AssetDefinition(#[from] AssetDefinitionError),
}

pub async fn exec(
    id: &Option<String>,
    source: &SourceArg,
    force: bool,
    include: Vec<String>,
    exclude: Option<Vec<String>>,
) -> Result<(), InstallError> {
    if let Some(id) = id {
        let mut config = Config::get()?;

        let provider = match source {
            SourceArg::AssetLib => AssetProvider::AssetLib { id: id.to_string() },
            SourceArg::Git => AssetProvider::Git {
                repo_url: id.to_string(),
            },
            SourceArg::Local => {
                return Err(InstallError::AssetProvider(
                    AssetProviderError::NotSupported,
                ))
            }
        };

        let asset_metadata = provider.info().await;

        let asset_def = AssetDefinition {
            id: id.to_string(),
            source: source.clone(),
            cache: Default::default(),
            include,
            exclude,
            metadata: asset_metadata,
        };

        if let Some(existing) = config.get_asset_info(id).cloned() {
            if force {
                info!("Updated {}...", asset_def.id);
                asset::uninstall(existing.id.clone())?;
                config.add_asset(id.to_string(), asset_def.clone())?;
            } else {
                warn!(
                    "Asset with ID '{}' is already added. Use --force to update configuration.",
                    id
                );
                return Ok(());
            }
        } else {
            config.add_asset(id.to_string(), asset_def.clone())?;
            info!("Added {} to project.", asset_def);
        }
    }

    let assets = Config::get()?.asset_definitions;
    let progress = MultiProgress::new();

    if assets.is_empty() {
        warn!("No assets are added. Try 'godam install <ID>'");
        return Ok(());
    }

    let mut tasks = JoinSet::new();

    for asset in assets.into_values() {
        let pb = progress.add(ProgressBar::new_spinner().with_style(progress_style()));

        if asset.is_installed()? {
            if force {
                asset::uninstall(asset.id.clone())?;
            } else {
                pb.subtle("Already installed", &asset.to_string());
                continue;
            }
        }

        tasks.spawn(async move {
            pb.enable_steady_tick(std::time::Duration::from_millis(100));

            match asset.install(&pb).await {
                Ok(_) => pb.complete("Installed", &asset.to_string()),
                Err(e) => pb.fail(&asset.to_string(), &e.to_string()),
            };
        });
    }

    tasks.join_all().await;

    Ok(())
}
