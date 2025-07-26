use crate::config::zuko_context::ZukoContext;
use inquire::{Text, Select, Confirm, MultiSelect};
use crate::types::{DifficultyFilter, Difficulty};
use crate::db::ZUKO_USER_DB;
use libsql::Builder;    


pub async fn execute(context: &mut ZukoContext) {
    /*
    TODO: sync remote zuko DB
    */

    let username = Text::new("👤 Your name?")
        .prompt()
        .unwrap();

    let project_name = Text::new("🗂️  Project name?")
        .prompt()
        .unwrap();

    let git_init = Confirm::new("🌀 Initialize Git?")
        .with_default(context.init_git)
        .prompt()
        .unwrap();

    let track_progress = Confirm::new("📈 Track progress?")
        .with_default(context.track_progress)
        .prompt()
        .unwrap();

    let editor = Text::new("📝 Editor of choice?")
        .with_default("code")
        .prompt()
        .unwrap();

    let preferred_language = MultiSelect::new("🧠 Preferred language?", context.preferred_language_list.clone())
        .prompt()
        .unwrap();
    

    let options = vec![
        DifficultyFilter::All,
        DifficultyFilter::Specific(Difficulty::Easy),
        DifficultyFilter::Specific(Difficulty::Medium),
        DifficultyFilter::Specific(Difficulty::Hard),
    ];

    let difficulty_mode = Select::new("🎚️ Default difficulty?", options)
        .prompt()
        .unwrap();

    // Update the context with the gathered information
    context.project_name = project_name;
    context.username = username;
    context.init_git = git_init;
    context.track_progress = track_progress;
    context.editor_cmd = editor;
    context.preferred_language_list = preferred_language;
    context.difficulty = difficulty_mode;
    context.project_root = std::env::current_dir().expect("Failed to get current directory");

    context.save_to_file().expect("Failed to save config to file");


    if track_progress {
        // Create user database
        let zuko_user_db = Builder::new_local(
            format!("file://{}", context.project_root.join("/db/zuko_user.db").display()).to_string(),
        )
        .build()
        .await
        .expect("Failed to build connection to zuko_user.db");

        let zuko_user_db_connection = zuko_user_db.connect().expect("Failed to connect to zuko_user.db");


        // get or initialize the ZUKO_USER_DB
        ZUKO_USER_DB
            .set(zuko_user_db_connection)
            .expect("ZUKO_USER_DB already initialized!");

        // create the user table if it doesn't exist
    } 

    // sync the zuko database
    crate::db::sync_zuko_db().await;
    println!("Zuko initialized successfully!");
}