use serde::{Deserialize, Serialize};

use super::asset_source::AssetSource;

#[derive(Serialize, Deserialize, Clone)]
pub struct AssetDefinition {
    pub id: String,
    pub title: String,
    pub source: AssetSource,
}
