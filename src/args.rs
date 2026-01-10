use clap::ValueEnum;
use serde::{Deserialize, Serialize};

#[derive(Default, Serialize, Deserialize, Clone, Debug, PartialEq, ValueEnum)]
#[clap(rename_all = "lower")]
pub enum CacheArg {
    #[default]
    Local,
    Global,
}

#[derive(Default, Serialize, Deserialize, Clone, Debug, PartialEq, ValueEnum)]
#[clap(rename_all = "lower")]
pub enum SourceArg {
    #[default]
    AssetLib,
    Local,
    Git,
}
