use libsql_client::{Client, Config};
use tokio::sync::OnceCell;

use utils::db::{get_zuko_cli_db_path, get_zuko_user_db_path};

pub struct DbManager {
    pub cli_client: Client,
    pub user_client: Client,
}

impl DbManager {
    pub async fn new() -> Self {
        let cli_client = Client::from_config(Config {
            url: get_zuko_cli_db_path().to_string_lossy().into(),
            auth_token: None,
        })
        .await
        .expect("Failed to connect to zuko-cli.db");

        let user_client = Client::from_config(Config {
            url: get_zuko_user_db_path().to_string_lossy().into(),
            auth_token: None,
        })
        .await
        .expect("Failed to connect to zuko-user.db");

        Self {
            cli_client,
            user_client,
        }
    }
}