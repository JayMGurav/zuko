use types::{Difficulty, DifficultyFilter};
use serde::{Deserialize, Serialize};
use std::fs;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AppConfig {
    pub preferred_lang: String,
    pub language_list: Vec<String>,
    pub editor_cmd: String,
    pub difficulty: DifficultyFilter,
}

impl AppConfig {
    pub fn from_config_file(config_path: &str) -> Option<Self> {
        // Try to read and parse the provided config file
        if let Ok(toml_content) = fs::read_to_string(config_path) {
            if let Ok(mut toml_config) = toml::from_str::<AppConfig>(&toml_content) {
                // project_root and cli_db_path will be empty if not present in the file
                return Some(toml_config);
            }
        } else {
            let default_path = "src/default_zuko.toml";
            //read from the default_path and return the default config
            let toml_content = fs::read_to_string(default_path).expect("Failed to read default config file");
            let mut toml_config = toml::from_str::<AppConfig>(&toml_content).expect("Failed to parse default config file");
            // project_root and cli_db_path will be empty if not present in the file
            return Some(toml_config);
        }
    }
}
