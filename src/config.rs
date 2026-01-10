use std::collections::BTreeMap;

use semver::Version;
use serde::{Deserialize, Serialize};
use thiserror::Error;

use crate::{
    assets::asset_definition::AssetDefinition,
    fs::{
        path::{get_addons_path, get_config_path, get_gitignore_path},
        ADDONS_GITIGNORE_CONTENT,
    },
    godot::{self, project::GodotProjectError},
};

#[derive(Error, Debug)]
pub enum ConfigError {
    #[error("IO error: {0}")]
    Io(#[from] std::io::Error),

    #[error("Parse error: {0}")]
    Parse(#[from] toml::de::Error),

    #[error("Parse error: {0}")]
    Serialize(#[from] toml::ser::Error),

    #[error("Godot error: {0}")]
    GodotError(#[from] GodotProjectError),
    #[error("Project is not initialized, try 'godam init'.")]
    Uninitialized,
}

#[derive(Serialize, Deserialize)]
pub struct Config {
    pub godot_version: Version,
    pub asset_definitions: BTreeMap<String, AssetDefinition>,
}

impl Config {
    pub fn get() -> Result<Self, ConfigError> {
        let config_path = get_config_path();
        let string = crate::fs::read_string(config_path).map_err(|_| ConfigError::Uninitialized)?;
        let config = toml::from_str(&string)?;

        Ok(config)
    }

    pub fn get_asset_info(&self, id: &str) -> Option<&AssetDefinition> {
        self.asset_definitions.get(id)
    }

    pub fn init() -> Result<(), ConfigError> {
        let version = godot::project::get_version()?;

        let config = Config {
            asset_definitions: BTreeMap::new(),
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

    pub fn add_asset(&mut self, id: String, asset: AssetDefinition) -> Result<(), ConfigError> {
        self.asset_definitions.insert(id, asset);
        self.save()
    }

    pub fn remove_asset(&mut self, id: &str) -> Result<Option<AssetDefinition>, ConfigError> {
        let removed_info = self.asset_definitions.remove(id);
        self.save()?;

        Ok(removed_info)
    }

    pub fn save(&self) -> Result<(), ConfigError> {
        let config_path = get_config_path();
        let str = toml::to_string(self)?;
        Ok(crate::fs::safe_write(config_path, str)?)
    }
}
