use crate::types::DifficultyFilter;
use serde::{Deserialize, Serialize};
use std::path::{PathBuf};
use derive_builder::Builder;
use dirs::home_dir; 
use std::fs;
use std::fs::create_dir_all;
use std::io::Write;


#[derive(Debug, Clone, Serialize, Deserialize, Builder)]
#[builder(default)]
pub struct ZukoContext {
    pub project_root: PathBuf,
    pub project_name: String,
    pub cli_dir: PathBuf,
    pub preferred_language_list: Vec<String>,
    pub editor_cmd: String,
    pub difficulty: DifficultyFilter,
    pub init_git: bool,
    pub track_progress: bool,
    pub username: String,
}

impl Default for ZukoContext {
    fn default() -> Self {

        let cli_dir = home_dir()
            .unwrap_or_else(|| PathBuf::from("."))
            .join(".zuko");
        

        if !cli_dir.exists() {
            create_dir_all(&cli_dir)
                .expect("Failed to create .zuko directory");
        }

        ZukoContext {
            project_root: PathBuf::new(),
            project_name: String::new(),
            cli_dir,
            preferred_language_list: vec![
                "java".to_string(),
                "python".to_string(),
                "rust".to_string(),
                "javascript".to_string(),
                "c++".to_string(),
                "ruby".to_string(),
                "go".to_string(),
            ],
            editor_cmd: "code".to_string(),
            difficulty: DifficultyFilter::default(),
            init_git: true,
            track_progress: false,
            username: String::new(),
        }
    }
}

impl ZukoContext {
    pub fn save_to_file(&self) -> std::io::Result<()> {
        // Serialize context to TOML
        let toml_string = toml::to_string_pretty(self)
            .expect("Failed to serialize ZukoContext");

        // Ensure .zuko directory exists
        if !self.cli_dir.exists() {
            create_dir_all(&self.cli_dir)?;
        }

        // Define config path
        let config_path = self.cli_dir.join("config.toml");

        // Write to config file
        let mut file = fs::File::create(&config_path)?;
        file.write_all(toml_string.as_bytes())?;

        println!("✅ Saved config to {}", config_path.display());

        Ok(())
    }

    fn load_context_from_file(cli_dir: &PathBuf) -> Option<ZukoContext> {
        let config_path = cli_dir.join("config.toml");
        let toml_str = fs::read_to_string(config_path).ok()?;
        toml::from_str(&toml_str).ok()
    }

    pub fn load_or_default() -> Self {
        let cli_dir = home_dir()
            .unwrap_or_else(|| PathBuf::from("."))
            .join(".zuko");

        if !cli_dir.exists() {
            create_dir_all(&cli_dir).expect("Failed to create .zuko directory");
        }


        if let Some(ctx) = Self::load_context_from_file(&cli_dir.join("config.toml")) {
            return ctx;
        }

        ZukoContextBuilder::default()
            .cli_dir(cli_dir)
            .build()
            .expect("Failed to build default context")
    }
}