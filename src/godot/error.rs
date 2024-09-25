use thiserror::Error;

#[derive(Error, Debug)]
pub enum GodotProjectError {
    #[error("Could not find project.godot file in working directory.")]
    ProjectNotFound,
    #[error("Could not parse version from project.godot file.")]
    VersionParse(#[from] semver::Error),
}

#[derive(Error, Debug)]
pub enum AssetLibraryError {
    #[error("API request failed: {0}")]
    Unhandled(#[from] reqwest::Error),
    #[error("Expected a valid ID (integer), found '{0}'")]
    InvalidId(String),
}
