use std::path::{Path, PathBuf};

use indicatif::ProgressBar;
use url::Url;

use crate::{
    args::{CacheArg, SourceArg},
    asset::{
        asset_archive::AssetArchive,
        cache,
        providers::asset_lib::{self, AssetLibAssetMetadata},
    },
    console::GodamProgressMessage,
};

pub struct InstallJob {
    args: InstallArgs,
    progress: ProgressBar,
    state: State,
    title: Option<String>,
}

impl InstallJob {
    pub fn new(args: InstallArgs, progress: ProgressBar, title: Option<String>) -> Self {
        Self {
            args,
            progress,
            state: State::CheckingCache {},
            title,
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
    CheckingCache,
    Resolving,
    Fetching {
        title: String,
        fetch_strategy: FetchStrategy,
    },
    Caching {
        title: String,
        cache_strategy: CacheStrategy,
    },
    Installing {
        path: PathBuf,
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

#[derive(Clone)]
enum CacheStrategy {
    FromBytes(Vec<u8>),
    FromRepo(Url),
}

impl InstallJob {
    pub async fn run(&mut self) {
        loop {
            let new_state = match &self.state {
                State::CheckingCache => self.check_cache().await,
                State::Resolving => self.resolve().await,
                State::Fetching {
                    title,
                    fetch_strategy,
                } => self.fetch(title.to_string(), fetch_strategy).await,
                State::Caching {
                    title,
                    cache_strategy,
                } => self.cache(title.to_string(), cache_strategy.clone()),
                State::Installing { path } => self.install(path),
                State::Skipped { title, reason } => {
                    self.progress
                        .subtle("Aborted", &format!("{}: {}", title, reason));
                    break;
                }
                State::Completed { title } => {
                    self.progress.complete("Installed", title);
                    break;
                }
                State::Failed { msg, reason } => {
                    self.progress.fail(msg, reason);
                    break;
                }
            };

            self.state = new_state;
        }
    }

    async fn check_cache(&self) -> State {
        let InstallArgs {
            id, cache, force, ..
        } = &self.args;

        self.progress.start("Checking Cache", id);

        match cache {
            CacheArg::Local => match cache::local::get(id) {
                Ok(Some(path)) => {
                    if *force {
                        State::Resolving {}
                    } else {
                        State::Installing { path }
                    }
                }
                Ok(None) => State::Resolving {},
                Err(e) => State::Failed {
                    msg: id.to_string(),
                    reason: format!("Failed to check local cache. ({e})"),
                },
            },
            CacheArg::Global => unimplemented!(),
        }
    }

    async fn resolve(&self) -> State {
        let progress = &self.progress;
        let InstallArgs { id, source, .. } = &self.args;

        progress.start("Resolving", id);

        match source {
            SourceArg::AssetLib => match asset_lib::get_asset_metadata(id).await {
                Ok(AssetLibAssetMetadata {
                    title,
                    download_url: Some(url),
                    ..
                }) => match Url::parse(&url) {
                    Ok(download_url) => {
                        let mut config = crate::config::Config::get().unwrap();
                        if let Err(e) = config.set_metadata(id, "title".to_string(), title.clone())
                        {
                            return State::Failed {
                                msg: id.to_string(),
                                reason: format!("Failed to save asset metadata. ({e})"),
                            };
                        }
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
                    reason: format!("No valid download URL found. Found ({download_url:?})"),
                },
                Err(e) => State::Failed {
                    msg: id.to_string(),
                    reason: format!("Failed to resolve asset id. ({e})"),
                },
            },
            _ => unimplemented!(),
        }
    }

    async fn fetch(&self, title: String, fetch_strategy: &FetchStrategy) -> State {
        let progress = &self.progress;

        progress.start("Fetching", &title);

        match fetch_strategy {
            FetchStrategy::AssetLib { download_url } => {
                progress.start("Downloading", &title);

                match asset_lib::download(download_url.as_str()).await {
                    Ok(bytes) => State::Caching {
                        title,
                        cache_strategy: CacheStrategy::FromBytes(bytes),
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

    fn cache(&self, title: String, cache_strategy: CacheStrategy) -> State {
        let InstallArgs { id, cache, .. } = &self.args;
        self.progress.start("Caching", &title);

        let cache_id = cache::CacheId::new(id);
        let cache_path = match cache {
            CacheArg::Local => crate::fs::path::get_cache_path(),
            CacheArg::Global => unimplemented!(),
        };

        match cache_strategy {
            CacheStrategy::FromBytes(zip_bytes) => {
                self.progress.start("Extracting to cache", &title);
                match AssetArchive::from_bytes(zip_bytes) {
                    Ok(mut archive) => match archive.extract_to_cache(cache_id, cache_path) {
                        Ok(path) => State::Installing { path },
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

    fn install(&self, path: &Path) -> State {
        let InstallArgs {
            include, exclude, ..
        } = &self.args;

        let title = self.title.clone().unwrap_or("MISSING TITLE".to_string());

        let mut project_scope_paths = Vec::new();

        for dir in walkdir::WalkDir::new(path) {
            match dir {
                Ok(entry) => {
                    let cached_path = entry.path().to_path_buf();

                    if cached_path.is_dir() {
                        continue;
                    }

                    let project_scope_path = match cached_path.strip_prefix(path) {
                        Ok(stripped) => stripped.to_path_buf(),
                        Err(e) => {
                            return State::Failed {
                                msg: title.clone(),
                                reason: format!("Failed to determine install path. ({e})"),
                            };
                        }
                    };

                    project_scope_paths.push(project_scope_path.clone());
                }
                Err(e) => {
                    return State::Failed {
                        msg: title.clone(),
                        reason: format!("Failed to read cache directory. ({e})"),
                    };
                }
            }
        }

        let paths_to_copy: Vec<PathBuf> = project_scope_paths
            .into_iter()
            .filter(|p| {
                include
                    .iter()
                    .any(|inc| p.to_string_lossy().starts_with(inc))
            })
            .filter(|p| {
                if let Some(exclude_patterns) = exclude {
                    !exclude_patterns
                        .iter()
                        .any(|exc| p.to_string_lossy().starts_with(exc))
                } else {
                    true
                }
            })
            .collect();

        for path_to_copy in paths_to_copy {
            let parent = path_to_copy
                .parent()
                .expect("cached file always has a parent");

            if parent.iter().count() > 0 && !parent.is_dir() {
                if let Err(e) = crate::fs::safe_create_dir(parent) {
                    return State::Failed {
                        msg: title.clone(),
                        reason: format!("Failed to create directory {}. ({e})", parent.display()),
                    };
                }
            }

            if let Err(e) = crate::fs::safe_copy(&path.join(&path_to_copy), &path_to_copy) {
                return State::Failed {
                    msg: title.clone(),
                    reason: format!("Failed to copy file. ({e})"),
                };
            }
        }

        State::Completed { title }
    }
}
