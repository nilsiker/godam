use std::str::FromStr;

use semver::Version;

pub fn get_project_version() -> Result<Version> {
    let file = std::fs::read_to_string("./project.godot")?;
    let string = file.lines().find(|line| line.starts_with(""));
}
