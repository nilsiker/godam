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
        let split = id.split(':').collect::<Vec<&str>>();

        if split.len() != 2 {
            return Err(AssetProviderError::NotSupported);
        }

        // let on a slice:
        let Some(reponame) = split.first() else {
            return Err(AssetProviderError::NotSupported);
        };
        let Some(branch) = split.get(1) else {
            return Err(AssetProviderError::NotSupported);
        };

        let github_url = format!("https://github.com/{reponame}/archive/refs/heads/{branch}.zip");

        Ok(Some(AssetMetadata {
            asset_id: id.to_string(),
            title: reponame.replace("/", "."),
            download_url: Some(github_url),
        }))
    }

    async fn download(&self, id: &str) -> Result<AssetBlob, AssetProviderError> {
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

        let (heads_or_tags, branch_or_tag) = if version.starts_with('#') {
            ("tags", &version[1..])
        } else {
            ("heads", *version)
        };

        let github_url = format!(
            "https://github.com/{reponame}/archive/refs/{heads_or_tags}/{branch_or_tag}.zip"
        );

        let bytes = web_requests::get_blob(Url::parse(&github_url)?).await?;

        Ok(AssetBlob { bytes })
    }
}
