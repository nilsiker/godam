use semver::Version;
use serde::{Deserialize, Serialize};
use thiserror::Error;

use crate::{
    assets::AssetInfo,
    fs::{
        path::{get_addons_path, get_config_path, get_gitignore_path},
        ADDONS_GITIGNORE_CONTENT,
    },
    godot,
};

#[derive(Error, Debug)]
pub enum ConfigError {
    #[error("Asset {0} is not present in configuration.")]
    AssetNotFound(String),

    #[error("IO error: {0}")]
    Io(#[from] std::io::Error),

    #[error("Parse error: {0}")]
    Parse(#[from] toml::de::Error),

    #[error("Parse error: {0}")]
    Serialize(#[from] toml::ser::Error),

    #[error("Godot error: {0}")]
    GodotError(#[from] godot::GodotError),
    #[error("Project is not initialized, try 'godam init'.")]
    Uninitialized,
}

#[derive(Serialize, Deserialize)]
pub struct Config {
    pub godot_version: Version,
    pub assets: Vec<AssetInfo>,
}
impl Config {
    pub fn get() -> Result<Self, ConfigError> {
        let config_path = get_config_path();
        let string =
            crate::fs::read_string(&config_path).map_err(|_| ConfigError::Uninitialized)?;
        let config = toml::from_str(&string)?;

        Ok(config)
    }

    pub fn asset(&self, id: &str) -> Result<&AssetInfo, ConfigError> {
        self.assets
            .iter()
            .find(|a| a.asset_id == id)
            .ok_or(ConfigError::AssetNotFound(id.to_string()))
    }

    fn asset_mut(&mut self, id: &str) -> Option<&mut AssetInfo> {
        self.assets.iter_mut().find(|a| a.asset_id == id)
    }

    pub fn init() -> Result<(), ConfigError> {
        let version = godot::get_project_version()?;

        let config = Config {
            assets: vec![],
            godot_version: version,
        };

        let contents = toml::to_string(&config)?;

        let addons_path = get_addons_path();
        if !crate::fs::exists(addons_path)? {
            crate::fs::safe_create_dir(addons_path)?;
        }
        crate::fs::safe_write(get_config_path(), contents)?;
        crate::fs::safe_write(get_gitignore_path(), ADDONS_GITIGNORE_CONTENT)?;

        Ok(())
    }

    pub fn add_asset(&mut self, asset: AssetInfo) -> Result<(), ConfigError> {
        if self.assets.contains(&asset) {
            println!("Asset is already registered. Skipping...");
        } else {
            self.assets.push(asset);
            self.save()?
        }
        Ok(())
    }

    pub fn remove_asset(&mut self, id: &str) -> Result<(), ConfigError> {
        match self.assets.iter().position(|asset| asset.asset_id == id) {
            Some(index) => {
                self.assets.remove(index);
                self.save()
            }
            None => Err(ConfigError::AssetNotFound(id.to_string())),
        }
    }

    pub fn save(&self) -> Result<(), ConfigError> {
        let config_path = get_config_path();
        let str = toml::to_string_pretty(self)?;
        Ok(crate::fs::safe_write(&config_path, str)?)
    }

    pub fn register_install_folder(
        &mut self,
        id: &str,
        install_folder: String,
    ) -> Result<(), ConfigError> {
        match self.asset_mut(id) {
            Some(asset) => asset.install_folder = Some(install_folder),
            None => println!("Asset ID not found in configuration"),
        }
        self.save()
    }
}
