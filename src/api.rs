use anyhow::{anyhow, Result};
use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, Clone)]
pub struct Asset {
    asset_id: String,
    title: String,
}

#[derive(Deserialize, Serialize, Clone)]
pub struct GodotResponse {
    result: Vec<Asset>,
}

#[derive(Debug)]
pub enum GodotApiError {
    NoAddonsFound(String),
    MultipleAddonsFound(String, Vec<String>),
}
impl std::fmt::Display for GodotApiError{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let msg = match self {
            GodotApiError::NoAddonsFound(filter) => format!("No addon with filter {filter}"),
            GodotApiError::MultipleAddonsFound(filter, found) => {
                format!("Could not unambiguously filter out the addon. Results using filter \"{filter}\":\n{found:#?}")
            }
        };
        write!(f, "{msg}")
    }
}

pub async fn find_addon(name: &str) -> Result<Asset> {
    let request_url = format!("https://godotengine.org/asset-library/api/asset?filter={name}");
    let response = reqwest::get(&request_url).await?;

    // println!("{:#?}", response.text().await?);

    let godot_response = response.json::<GodotResponse>().await?;
    let result = godot_response.result;

    match result.len() {
        0 => Err(anyhow!(GodotApiError::NoAddonsFound(name.to_string()))),
        // 1 => return Ok(result[0].clone()),
        _ => Err(anyhow!(GodotApiError::MultipleAddonsFound(
            name.to_string(),
            result.into_iter().map(|asset| asset.title).collect()
        ))),
    }
}
