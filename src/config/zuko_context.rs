
use serde::{Deserialize, Serialize};
use std::fs;
use std::path::Path;
use config::{AppConfig};
use db::DbManager;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ZukoContext {
   pub app_config: AppConfig,
   pub db_manager: DbManager,
   #[serde(default)] //TODO: add manual default later
    pub project_root: String,
    #[serde(default)] //TODO: add manual default later
    pub cli_db_path: String,
}


impl ZukoContext {
    pub fn new(project_root: String, cli_db_path: String) -> Self {
        // Initialize app_config with project root parameter and returned values
        let user_config_file_full_path = project_root.join("zuko.toml");
        let app_config = AppConfig::from_config_file(&user_config_file_full_path)
            .unwrap_or_else(|| AppConfig::default());

        let db_manager = DbManager::new().await;

        Self {
            app_config,
            db_manager,
            project_root,
            cli_db_path,
        }
    }
}
