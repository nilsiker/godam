use crate::{
    assets::AssetInfo,
    config::{Config, ConfigError},
    info,
};

pub fn exec() -> Result<(), ConfigError> {
    let config = Config::get()?;

    let longest_id_length = config
        .asset_infos
        .iter()
        .max_by(|a, b| a.0.len().cmp(&b.0.len()))
        .expect("one is longest")
        .0
        .len();

    let longest_title = config
        .asset_infos
        .iter()
        .max_by(|a, b| a.1.title.len().cmp(&b.1.title.len()))
        .expect("one is longest")
        .1
        .title
        .len();

    for (id, AssetInfo { title, .. }) in config.asset_infos {
        info!(
            "{id:>width$}: {title:<title_width$}",
            width = longest_id_length,
            title_width = longest_title
        )
    }

    Ok(())
}
