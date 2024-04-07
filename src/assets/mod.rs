pub mod service;

use serde::{Deserialize, Serialize};
use thiserror::Error;

#[derive(Debug, Error)]
pub enum AssetError {
    #[error("No addon with filter {0}")]
    NoAddonsFound(String),
    #[error(
        "Could not unambiguously find addon using \"{filter}\". Candidates found: {candidates}"
    )]
    MultipleAddonsFound { filter: String, candidates: String },
}

#[derive(Deserialize, Serialize, Clone, Debug, PartialEq)]
pub struct Asset {
    pub asset_id: String,
    pub title: String,
    pub browse_url: Option<String>,
    pub download_commit: Option<String>,
}

