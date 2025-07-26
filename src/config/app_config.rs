use crate::types::DifficultyFilter;
use serde::{Deserialize, Serialize};
use std::fs;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AppConfig {
    pub preferred_language_list: Vec<String>,
    pub editor_cmd: String,
    pub difficulty: DifficultyFilter,
}

impl AppConfig {
    pub fn from_config_file(config_path: &str) -> Option<Self> {
        if let Ok(toml_content) = fs::read_to_string(config_path) {
            toml::from_str::<AppConfig>(&toml_content).ok()
        } else {
            let default_path = "src/default_zuko.toml"; //TODO to be cli execuatable path
            let toml_content = fs::read_to_string(default_path).expect("Failed to read default config file");
            toml::from_str::<AppConfig>(&toml_content).ok()
        }
    }
    pub fn default() -> Self {
        let default_path = "src/default_zuko.toml"; //TODO to be cli execuatable path
        let toml_content = fs::read_to_string(default_path).expect("Failed to read default config file");

        return toml::from_str::<AppConfig>(&toml_content).ok()
            .unwrap_or_else(|| AppConfig {
                preferred_language_list: vec!["python".to_string()],
                editor_cmd: "code".to_string(),
                difficulty: DifficultyFilter::default(),
            });
    }
}

