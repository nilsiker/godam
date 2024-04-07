use anyhow::{anyhow, Result};
use semver::Version;
use serde::{Deserialize, Serialize};
use serde_json::{de, ser};
use thiserror::Error;

use crate::{assets::Asset, godot};

const CONFIG_RELATIVE_PATH: &str = "./godam.json";

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
}

#[derive(Serialize, Deserialize)]
pub struct Config {
    pub godot_version: Version,
    pub assets: Vec<Asset>,
}
impl Config {
    pub fn get() -> Result<Self> {
        let json_path = std::env::current_dir()?.join(CONFIG_RELATIVE_PATH);
        let string = std::fs::read_to_string(json_path).map_err(|_| ConfigError::NotFound)?;

        Ok(de::from_str(&string)?)
    }

    pub fn init() -> Result<()> {
        let version = godot::get_project_version()?;

        let config = Config {
            assets: vec![],
            godot_version: version,
        };

        let contents = ser::to_string_pretty(&config)?;

        let json_path = std::env::current_dir()?.join(CONFIG_RELATIVE_PATH);
        std::fs::write(json_path, contents)?;

        Ok(())
    }

    pub fn add_asset(&mut self, asset: Asset) -> Result<()> {
        match self.assets.contains(&asset) {
            true => Err(anyhow!(ConfigError::FailedAdd(format!(
                "{} is already registered",
                asset.title
            )))),
            false => {
                self.assets.push(asset);
                self.save()
            }
        }
    }

    pub fn remove_asset(&mut self, name: &str) -> Result<()> {
        match self.assets.iter().position(|asset| asset.title == name) {
            Some(index) => {
                self.assets.remove(index);
                self.save()
            }
            None => Err(anyhow!(ConfigError::FailedRemove(format!(
                "No asset with name {name} found."
            )))),
        }
    }

    fn save(&self) -> Result<()> {
        let json_path = std::env::current_dir()?.join(CONFIG_RELATIVE_PATH);
        let str = ser::to_string_pretty(self)?;
        Ok(std::fs::write(json_path, str)?)
    }
}
