use crate::{
    assets::AssetInfo,
    config::{Config, ConfigError},
    info,
};

pub fn exec() -> Result<(), ConfigError> {
    let config = Config::get()?;

    let longest_id_length = config
        .assets
        .iter()
        .max_by(|a, b| a.asset_id.len().cmp(&b.asset_id.len()))
        .expect("one is longest")
        .asset_id
        .len();

    let longest_title = config
        .assets
        .iter()
        .max_by(|a, b| a.title.len().cmp(&b.title.len()))
        .expect("one is longest")
        .title
        .len();

    for AssetInfo {
        asset_id: id,
        title,
        ..
    } in config.assets
    {
        info!(
            "{id:>width$}: {title:<title_width$}",
            width = longest_id_length,
            title_width = longest_title
        )
    }

    Ok(())
}
