use std::{fs, path::PathBuf};

use thiserror::Error;

#[derive(Error, Debug)]
pub enum PluginConfigError {
    #[error(transparent)]
    Io(#[from] std::io::Error),
    #[error("Plugin configuration file does not contain a name field")]
    NameNotFound,
}

pub struct PluginConfig {
    name: String,
}

impl TryFrom<PathBuf> for PluginConfig {
    type Error = PluginConfigError;

    fn try_from(value: PathBuf) -> Result<Self, Self::Error> {
        let file = fs::read_to_string(&value)?;

        let name = file
            .lines()
            .find(|line| line.starts_with("name="))
            .map(|line| line.trim_start_matches("name="))
            .map(|quoted_name| quoted_name.trim_matches('"').to_string())
            .ok_or(PluginConfigError::NameNotFound)?;

        Ok(PluginConfig { name })
    }
}

impl PluginConfig {
    pub fn name(&self) -> &str {
        &self.name
    }
}

