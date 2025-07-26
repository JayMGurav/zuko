use std::path::PathBuf;
use tokio::sync::OnceCell;
use libsql::Connection;
use libsql::{Builder};

pub struct DbManager {
    pub cli_client: Connection,
    pub user_client: Connection,
}

static DB_MANAGER: OnceCell<DbManager> = OnceCell::const_new();

impl DbManager {
    pub async fn new(cli_db_path: PathBuf, user_db_path: PathBuf) -> Self {
        if !cli_db_path.exists() {
            panic!("CLI database does not exist: {}", cli_db_path.display());
        }

        if !user_db_path.exists() {
            panic!("User database  does not exist: {}", user_db_path.display());
        }


        let cli_db = Builder::new_remote_replica(
            format!("file://{}", cli_db_path.display()).to_string(),
            "libsql://...".to_string(),
            "...".to_string()
        )
        .build()
        .await.expect("Failed to build connection to zuko-cli.db");
    
        let user_db = Builder::new_local(
            format!("file://{}", user_db_path.display())
        )
            .build()
            .await
            .expect("Failed to build connection to local user.db");

        let cli_client = cli_db.connect().expect("Failed to connect to zuko-cli.db");

        let user_db_conn = user_db.connect().expect("Failed to connect to local user.db");

        Self {
            cli_client: cli_client,
            user_client: user_db_conn,
        }
    }

    pub async fn init(cli_db_path: PathBuf, user_db_path: PathBuf) -> &'static DbManager {
        DB_MANAGER
            .get_or_init(|| async {
                DbManager::new(cli_db_path, user_db_path).await
            })
            .await
    }

    pub fn get() -> &'static DbManager {
        DB_MANAGER.get().expect("DbManager not initialized")
    }
}
