use std::str::FromStr;

use anyhow::{anyhow, Result};
use semver::Version;
use thiserror::Error;

const GODOT_PROJECT_LINE_START: &str = "config/features=PackedStringArray(";

#[derive(Error, Debug)]
pub enum GodotError {
    #[error("Could not find project.godot file in working directory.")]
    ProjectNotFound,
}

pub fn get_project_version() -> Result<Version> {
    let file =
        std::fs::read_to_string("./project.godot").map_err(|_| GodotError::ProjectNotFound)?;
    let string = file
        .lines()
        .find(|line| line.starts_with(GODOT_PROJECT_LINE_START));

    match string {
        Some(line) => {
            let first_quote = line.find('"').expect("first quote exists");
            let second_quote = first_quote
                + line[first_quote + 1..]
                    .find('"')
                    .expect("matching closing quote");
            let mut version_str = line[first_quote + 1..=second_quote].to_string();
            if version_str.len() == 3 {
                version_str += ".0";
            }
            Ok(Version::from_str(&version_str)?)
        }
        None => Err(anyhow!(GodotError::ProjectNotFound)),
    }
}
