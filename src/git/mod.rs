use anyhow::Result;
use git2::{Oid, Repository};

pub fn clone(name: &str, url: &str, commit: &str) -> Result<()> {
    let repo = Repository::clone(url, format!("./cache/{name}"))?;
    let commitish = Oid::from_str(commit)?;
    Ok(repo.set_head_detached(commitish)?)
}
