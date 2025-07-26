use crate::config::app_config::AppConfig;
use crate::db::db_manager::DbManager;

use serde::{Deserialize, Serialize};
use std::path::{PathBuf};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ZukoContext {
    pub app_config: AppConfig,
    pub project_root: PathBuf,
    pub cli_db_path: PathBuf,
}

impl ZukoContext {
    pub async fn new(project_root: PathBuf, cli_db_path: PathBuf) -> Self {
        let config_path = project_root.join("zuko.toml");
        let app_config = AppConfig::from_config_file(config_path.to_str().unwrap())
            .unwrap_or_else(|| AppConfig::default());

        DbManager::init(
            cli_db_path.clone(),
            project_root.join("db/zuko-user.db")
        ).await;

        Self {
            app_config,
            project_root,
            cli_db_path,
        }
    }

    pub fn get_db(&self) -> &'static DbManager {
        DbManager::get()
    }
}
