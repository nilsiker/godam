use std::path::{Path, PathBuf};

const CONFIG_PATH: &str = "./addons/godam.toml";
const GODOT_PROJECT_FILE_PATH: &str = "./project.godot";

const CACHE_PATH: &str = "./addons/.godam";

const ADDONS_PATH: &str = "./addons";
const ADDONS_GITIGNORE_PATH: &str = "./addons/.gitignore";

pub fn get_config_path() -> &'static Path {
    Path::new(CONFIG_PATH)
}

pub fn get_project_file_path() -> &'static Path {
    Path::new(GODOT_PROJECT_FILE_PATH)
}

pub fn get_cache_path() -> &'static Path {
    Path::new(CACHE_PATH)
}

pub fn get_cached_zip_path(id: &str) -> PathBuf {
    get_cache_path().join(id).with_extension("zip")
}

pub fn get_addons_path() -> &'static Path {
    Path::new(ADDONS_PATH)
}

pub fn get_gitignore_path() -> &'static Path {
    Path::new(ADDONS_GITIGNORE_PATH)
}

#[cfg(test)]
mod tests {
    use crate::fs::get_path_asserted_within_project;

    use super::*;

    #[test]
    fn config_path_is_within_working_directory() -> Result<(), Box<dyn std::error::Error>> {
        let config_path = get_config_path();
        let _ = get_path_asserted_within_project(config_path)?;

        Ok(())
    }

    #[test]
    fn project_file_path_is_within_working_directory() -> Result<(), Box<dyn std::error::Error>> {
        let project_path = get_project_file_path();
        let _ = get_path_asserted_within_project(project_path)?;

        Ok(())
    }

    #[test]
    fn addons_path_is_within_working_directory() -> Result<(), Box<dyn std::error::Error>> {
        let addons_path = get_addons_path();
        let _ = get_path_asserted_within_project(addons_path)?;

        Ok(())
    }

    #[test]
    fn addons_gitignore_path_is_within_working_directory() -> Result<(), Box<dyn std::error::Error>>
    {
        let gitignore_path = get_gitignore_path();
        let _ = get_path_asserted_within_project(gitignore_path)?;

        Ok(())
    }

    #[test]
    fn cache_path_is_within_working_directory() -> Result<(), Box<dyn std::error::Error>> {
        let cache_path = get_cache_path();
        let _ = get_path_asserted_within_project(cache_path)?;

        Ok(())
    }

    #[test]
    fn gitignore_is_within_working_directory() -> Result<(), Box<dyn std::error::Error>> {
        let gitignore_path = get_gitignore_path();
        let _ = get_path_asserted_within_project(gitignore_path)?;
        Ok(())
    }

    #[test]
    fn cached_zip_path_is_within_working_directory() -> Result<(), Box<dyn std::error::Error>> {
        let cached_zip_path = get_cached_zip_path("1234");
        let _ = get_path_asserted_within_project(&cached_zip_path)?;

        Ok(())
    }
}
