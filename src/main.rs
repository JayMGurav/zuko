mod config;
mod db;
mod types;
mod utils;

use std::path::PathBuf;
use config::zuko_context::ZukoContext;
use utils::db::{get_zuko_cli_db_path};

#[tokio::main]
async fn main() {
    let project_root = PathBuf::from("/path/to/project");
    let cli_db_path = get_zuko_cli_db_path();

    let context = ZukoContext::new(project_root, cli_db_path).await;

    // Pass context to command handlers
    // run_command(&context);
}
