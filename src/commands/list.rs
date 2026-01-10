use crate::{
    config::{Config, ConfigError},
    info,
};

pub fn exec() -> Result<(), ConfigError> {
    let config = Config::get()?;

    if config.asset_definitions.is_empty() {
        info!("No assets found.");
        return Ok(());
    }

    let longest_id_length = config
        .asset_definitions
        .iter()
        .max_by(|a, b| a.0.len().cmp(&b.0.len()))
        .expect("one is longest")
        .0
        .len();

    for (id, asset_def) in config.asset_definitions {
        let title = asset_def.to_string();
        info!("{id:>width$}: {title}", width = longest_id_length,)
    }

    Ok(())
}
