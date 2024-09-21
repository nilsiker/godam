use serde::{Deserialize, Serialize};
use thiserror::Error;


#[derive(Debug, Error)]
pub enum AssetError {

}

#[derive(Deserialize, Serialize, Clone, Debug, PartialEq)]
pub struct Asset {
    pub asset_id: String,
    pub title: String,
    pub download_url: String,
}

pub fn is_asset_installed(asset: &Asset) -> bool {
    true
}