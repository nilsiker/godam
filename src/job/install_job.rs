use std::path::PathBuf;

use indicatif::ProgressBar;
use url::Url;

use crate::{
    args::{CacheArg, SourceArg},
    asset::{
        cache::asset_archive::AssetArchive,
        providers::asset_lib::{self, AssetLibAssetMetadata},
    },
    console::GodamProgressMessage,
    warn,
};

pub struct InstallJob {
    args: InstallArgs,
    progress: ProgressBar,
    state: State,
}
impl InstallJob {
    pub fn new(args: InstallArgs, progress: ProgressBar) -> Self {
        Self {
            args,
            progress,
            state: State::Resolving {},
        }
    }
}

pub struct InstallArgs {
    pub id: String,
    pub source: SourceArg,
    pub cache: CacheArg,
    pub force: bool,
    pub include: Vec<String>,
    pub exclude: Option<Vec<String>>,
}

enum State {
    Resolving {},
    Fetching {
        title: String,
        fetch_strategy: FetchStrategy,
    },
    Caching {
        title: String,
        path: PathBuf,
    },
    Installing {
        title: String,
        install_strategy: InstallStrategy,
    },
    Skipped {
        title: String,
        reason: String,
    },
    Completed {
        title: String,
    },
    Failed {
        msg: String,
        reason: String,
    },
}

enum FetchStrategy {
    AssetLib {
        download_url: Url,
    },
    Git {
        repo_url: String,
        branch: Option<String>,
    },
}

enum InstallStrategy {
    ExtractZip { zip_bytes: Vec<u8> },
    FromCacheZip { cache_zip_path: PathBuf },
    FromCacheDirectory { cache_path: PathBuf },
}

impl InstallJob {
    pub async fn run(mut self) -> Result<(), Box<dyn std::error::Error>> {
        let InstallArgs {
            id,
            source,
            cache,
            force,
            include,
            exclude,
        } = &self.args;

        let progress = self.progress;

        loop {
            self.state = match self.state {
                State::Resolving {} => {
                    progress.start("Resolving", id);

                    match source {
                        SourceArg::AssetLib => match asset_lib::get_asset_metadata(id).await {
                            Ok(AssetLibAssetMetadata {
                                title,
                                download_url: Some(url),
                                ..
                            }) => match Url::parse(&url) {
                                Ok(download_url) => {
                                    let fetch_strategy = FetchStrategy::AssetLib { download_url };
                                    State::Fetching {
                                        title,
                                        fetch_strategy,
                                    }
                                }
                                Err(e) => State::Failed {
                                    msg: id.to_string(),
                                    reason: format!("Invalid download URL format. ({e})"),
                                },
                            },
                            Ok(AssetLibAssetMetadata { download_url, .. }) => State::Failed {
                                msg: id.to_string(),
                                reason: format!(
                                    "No valid download URL found. Found ({download_url:?})"
                                ),
                            },
                            Err(e) => State::Failed {
                                msg: id.to_string(),
                                reason: format!("Failed to resolve asset id. ({e})"),
                            },
                        },
                        _ => unimplemented!(),
                    }
                }
                State::Fetching {
                    title,
                    fetch_strategy,
                } => {
                    progress.start("Fetching", &title);

                    match fetch_strategy {
                        FetchStrategy::AssetLib { download_url } => {
                            progress.start("Downloading", &title);

                            match asset_lib::download(download_url.as_str()).await {
                                Ok(bytes) => State::Installing {
                                    title,
                                    install_strategy: InstallStrategy::ExtractZip {
                                        zip_bytes: bytes,
                                    },
                                },
                                Err(e) => State::Failed {
                                    msg: title,
                                    reason: format!("Failed to download asset. ({e})"),
                                },
                            }
                        }
                        _ => unimplemented!(),
                    }
                }
                State::Caching { title, .. } => {
                    progress.start("Caching", &title);

                    unimplemented!()
                }
                State::Installing {
                    title,
                    install_strategy,
                } => {
                    progress.complete("Installing", &title);

                    match install_strategy {
                        InstallStrategy::ExtractZip { zip_bytes } => {
                            progress.start("Extracting", &title);

                            match AssetArchive::from_bytes(zip_bytes) {
                                Ok(mut archive) => match archive.extract(include, exclude) {
                                    Ok(_) => State::Completed { title },
                                    Err(e) => State::Failed {
                                        msg: title,
                                        reason: format!("Failed to extract asset archive. ({e})"),
                                    },
                                },
                                Err(e) => State::Failed {
                                    msg: title,
                                    reason: format!("Failed to read asset archive. ({e})"),
                                },
                            }
                        }
                        _ => unimplemented!(),
                    }
                }
                State::Skipped { title, reason } => {
                    progress.subtle("Aborted", &format!("{}: {}", title, reason));
                    break;
                }
                State::Completed { title } => {
                    progress.complete("Installed", &title);
                    break;
                }
                State::Failed { msg, reason } => {
                    progress.fail(&msg, &reason);
                    break;
                }
            };
        }

        Ok(())
    }
}
