use semver::Version;
use std::str::FromStr;

use crate::fs::path::get_project_file_path;

use super::error::GodotProjectError;

const GODOT_PROJECT_LINE_START: &str = "config/features=PackedStringArray(";

pub fn get_version() -> Result<Version, GodotProjectError> {
    let file =
        crate::fs::read_string(get_project_file_path()).map_err(|_| GodotProjectError::ProjectNotFound)?;
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
            let version = Version::from_str(&version_str)?;
            Ok(version)
        }
        None => Err(GodotProjectError::ProjectNotFound),
    }
}
