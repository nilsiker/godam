use clap::ValueEnum;
use serde::{Deserialize, Serialize};

#[derive(Default, Serialize, Deserialize, Clone, Debug, PartialEq, ValueEnum)]
pub enum AssetSource {
    #[default]
    AssetLib,
    Github,
}
