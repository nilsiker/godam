use anyhow::{anyhow, Result};
use semver::Version;
use serde::{Deserialize, Serialize};
use thiserror::Error;

use crate::{assets::AssetInfo, godot};

pub const ADDONS_RELATIVE_PATH: &str = "./addons";
const CONFIG_RELATIVE_PATH: &str = "./addons/godam.toml";
const ADDONS_GITIGNORE_PATH: &str = "./addons/.gitignore";
const ADDONS_GITIGNORE_CONTENT: &str = "*\n!.gitignore\n!godam.toml\n.godam";

#[derive(Error, Debug)]
pub enum ConfigError {
    #[error("No configuration found. Ensure the project is initialized.")]
    NotFound,
    #[error("The configuration format is invalid.")]
    InvalidFormat,
    #[error("Could not remove asset. {0}.")]
    FailedRemove(String),
    #[error("Could not add asset. {0}.")]
    FailedAdd(String),
    #[error("Could not uninstall asset. {0}.")]
    FailedUninstall(String),
}

#[derive(Serialize, Deserialize)]
pub struct Config {
    pub godot_version: Version,
    pub assets: Vec<AssetInfo>,
}
impl Config {
    pub fn get() -> Result<Self> {
        let toml_path = std::env::current_dir()?.join(CONFIG_RELATIVE_PATH);
        let string = std::fs::read_to_string(toml_path).map_err(|_| ConfigError::NotFound)?;

        Ok(toml::from_str(&string)?)
    }

    pub fn asset(&self, id: &str) -> Option<&AssetInfo> {
        self.assets.iter().find(|a| a.asset_id == id)
    }

    fn asset_mut(&mut self, id: &str) -> Option<&mut AssetInfo> {
        self.assets.iter_mut().find(|a| a.asset_id == id)
    }

    pub fn init() -> Result<()> {
        let version = godot::get_project_version()?;

        let config = Config {
            assets: vec![],
            godot_version: version,
        };

        let contents = toml::to_string(&config)?;

        if !std::fs::exists(ADDONS_RELATIVE_PATH)? {
            std::fs::create_dir(ADDONS_RELATIVE_PATH)?;
        }
        std::fs::write(CONFIG_RELATIVE_PATH, contents)?;
        std::fs::write(ADDONS_GITIGNORE_PATH, ADDONS_GITIGNORE_CONTENT)?;

        Ok(())
    }

    pub fn add_asset(&mut self, asset: AssetInfo) -> Result<()> {
        if self.assets.contains(&asset) {
            println!("Asset is already registered. Skipping...");
        } else {
            self.assets.push(asset);
            self.save()?
        }
        Ok(())
    }

    pub fn remove_asset(&mut self, id: &str) -> Result<()> {
        match self.assets.iter().position(|asset| asset.asset_id == id) {
            Some(index) => {
                self.assets.remove(index);
                self.save()
            }
            None => Err(anyhow!(ConfigError::FailedRemove(format!(
                "No asset with id {id} found."
            )))),
        }
    }

    pub fn save(&self) -> Result<()> {
        let toml_path = std::env::current_dir()?.join(CONFIG_RELATIVE_PATH);
        let str = toml::to_string_pretty(self)?;
        Ok(std::fs::write(toml_path, str)?)
    }

    pub fn register_install_folder(&mut self, id: &str, install_folder: &str) {
        match self.asset_mut(id) {
            Some(asset) => asset.install_folder = Some(install_folder.to_string()),
            None => println!("Asset ID not found in configuration"),
        }
        self.save().expect("can save config");
    }

    pub fn contains_asset(&self, id: &str) -> bool {
        self.asset(id).is_some()
    }
}
