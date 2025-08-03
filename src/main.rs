mod commands;
mod config;
mod db;
mod types;
mod utils;
mod ui;

use clap::{Parser, Subcommand};
use db::{ZUKO_DB, ZUKO_DATABASE};
use libsql::Builder;
use config::db::{TURSO_DB_URL, TURSO_DB_TOKEN};

use crate::config::zuko_context::{ZukoContext};

#[derive(Parser)]
#[command(name = "zuko")]
#[command(
    about = "CLI to upgrade your DSA practice",
    long_about = "Zuko is a blazing-fast, is a terminal-first CLI tool to supercharge your daily Data Structures and Algorithms (DSA) practice. It helps you stay consistent with your Data Structures & Algorithms (DSA) practice. With powerful UI, fast fuzzy search, and automatic boilerplate generation, Zuko brings discipline and speed to your daily routine."
)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// Initialize zuko in the current directory
    Init,

    /// List questions optionally filtered by topic, difficulty, or solved status
    List {
        #[arg(long)]
        topic_slug: Option<String>,

        #[arg(long)]
        difficulty: Option<String>,

        #[arg(long, default_value_t = false)]
        solved: bool,
    },

    /// Sync with the remote zuko database
    Sync,
}

#[tokio::main]
async fn main() {
    let cli = Cli::parse();

    let mut context = ZukoContext::load_or_default();

    // print context value to console
    // { project_root: "", project_name: "", cli_dir: "/Users/jaygurav/.zuko", preferred_language_list: ["java", "python", "rust", "javascript", "c++", "ruby", "go"], editor_cmd: "code", difficulty: All, init_git: true, track_progress: false, username: "" }

    // initialize zuko db
    let zuko_db = Builder::new_remote_replica(
        format!("{}", context.cli_dir.join("db/zuko.db").display()).to_string(),
        TURSO_DB_URL.to_string(), //remote DB connection string
        TURSO_DB_TOKEN.to_string(), //remote DB encryption string
    )
    .build()
    .await
    .expect("Failed to build connection to zuko-cli.db");

    let zuko_db_connection = zuko_db.connect().expect("Failed to connect to zuko-cli.db");

    ZUKO_DATABASE
        .set(zuko_db)
        .expect("ZUKO_DATABASE already initialized!");

    ZUKO_DB
        .set(zuko_db_connection)
        .expect("ZUKO_DB already initialized!");

    match &cli.command {
        Commands::Init => {
            // Handle the init command
            commands::init::execute(&mut context).await;
        }
        Commands::List {
            topic_slug,
            difficulty,
            solved,
        } => {
            // Handle the list command
            commands::list::execute(&context, topic_slug.clone(), difficulty.clone(), *solved).await;
        }
        Commands::Sync => {
            // Handle the sync command
            commands::sync::execute().await;
        }
    }
}
