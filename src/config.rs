use std::collections::HashMap;

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
    GodotError(#[from] godot::error::GodotProjectError),
    #[error("Project is not initialized, try 'godam init'.")]
    Uninitialized,
}

#[derive(Serialize, Deserialize)]
pub struct Config {
    pub godot_version: Version,
    pub assets: Vec<AssetInfo>,
    pub install_folders: HashMap<String, String>,
}

impl Config {
    pub fn get() -> Result<Self, ConfigError> {
        let config_path = get_config_path();
        let string = crate::fs::read_string(config_path).map_err(|_| ConfigError::Uninitialized)?;
        let config = toml::from_str(&string)?;

        Ok(config)
    }

    pub fn get_asset(&self, id: &str) -> Result<&AssetInfo, ConfigError> {
        self.assets
            .iter()
            .find(|a| a.asset_id == id)
            .ok_or(ConfigError::AssetNotFound(id.to_string()))
    }

    pub fn get_install_folder(&self, asset_id: &str) -> Option<&String> {
        self.install_folders.get(asset_id)
    }

    pub fn set_install_folder(
        &mut self,
        id: &str,
        install_folder: String,
    ) -> Result<(), ConfigError> {
        self.install_folders.insert(id.to_string(), install_folder);
        self.save()
    }

    pub fn init() -> Result<(), ConfigError> {
        let version = godot::project::get_version()?;

        let config = Config {
            assets: vec![],
            godot_version: version,
            install_folders: HashMap::new(),
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

    pub fn remove_asset(&mut self, asset_id: &str) -> Result<(), ConfigError> {
        match self
            .assets
            .iter()
            .position(|asset| asset.asset_id == asset_id)
        {
            Some(index) => {
                self.assets.remove(index);
            }
            None => return Err(ConfigError::AssetNotFound(asset_id.to_string())),
        };

        self.install_folders.remove(asset_id);
        self.save()
    }

    pub fn save(&self) -> Result<(), ConfigError> {
        let config_path = get_config_path();
        let str = toml::to_string_pretty(self)?;
        Ok(crate::fs::safe_write(config_path, str)?)
    }
}
