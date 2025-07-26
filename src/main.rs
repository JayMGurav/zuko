mod commands;
mod config;
mod db;
mod types;
mod utils;

use clap::{Parser, Subcommand};
use db::{ZUKO_DB, ZUKO_DATABASE};
use libsql::Builder;

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
        topic: Option<String>,

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

    // initialize zuko db
    let zuko_db = Builder::new_remote_replica(
        format!("file://{}", context.cli_dir.join("/db/zuko.db").display()).to_string(),
        "libsql://...".to_string(), //remote DB connection string
        "...".to_string(),          //remote DB encryption string
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

            println!("My Config: {:?}", context);
        }
        Commands::List {
            topic,
            difficulty,
            solved,
        } => {
            // Handle the list command
            commands::list::execute(&context, topic.clone(), difficulty.clone(), *solved).await;
        }
        Commands::Sync => {
            // Handle the sync command
            commands::sync::execute().await;
        }
    }
}
