use semver::Version;
use url::Url;

use crate::{
    asset_providers::{AssetBlob, AssetMetadata, AssetProvider, AssetProviderError},
    web_requests,
};

pub struct GitHub;

impl AssetProvider for GitHub {
    async fn query(
        &self,
        _name: &str,
        _version: Option<&Version>,
    ) -> Result<Vec<AssetMetadata>, AssetProviderError> {
        Err(AssetProviderError::NotSupported)
    }

    async fn lookup(&self, id: &str) -> Result<Option<AssetMetadata>, AssetProviderError> {
        let (reponame, github_url) = GitHub::get_info(id)?;

        Ok(Some(AssetMetadata {
            asset_id: id.to_string(),
            title: reponame.replace("/", "."),
            download_url: Some(github_url.to_string()),
        }))
    }

    async fn download(&self, id: &str) -> Result<AssetBlob, AssetProviderError> {
        let github_url = GitHub::get_info(id)?.1;
        let bytes = web_requests::get_blob(github_url).await?;

        Ok(AssetBlob { bytes })
    }
}

impl GitHub {
    /// Returns (reponame, download_url)
    fn get_info(id: &str) -> Result<(&str, Url), AssetProviderError> {
        let split = id.split(':').collect::<Vec<&str>>();

        if split.len() != 2 {
            return Err(AssetProviderError::NotSupported);
        }

        let Some(reponame) = split.first() else {
            return Err(AssetProviderError::NotSupported);
        };
        let Some(version) = split.get(1) else {
            return Err(AssetProviderError::NotSupported);
        };

        let github_url = if let Some(tag) = version.strip_prefix("tags") {
            format!("https://github.com/{reponame}/archive/refs/tags/{tag}.zip")
        } else {
            format!("https://github.com/{reponame}/archive/refs/heads/{version}.zip")
        };

        Ok((*reponame, Url::parse(&github_url)?))
    }
}
