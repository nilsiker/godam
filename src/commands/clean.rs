use thiserror::Error;

use crate::{cache, config};

#[derive(Error, Debug)]
pub enum CleanError {
    #[error(transparent)]
    Config(#[from] config::ConfigError),
    #[error(transparent)]
    Io(#[from] std::io::Error),
}

pub fn run() -> Result<(), CleanError> {
    cache::clear()?;
    Ok(())
}
