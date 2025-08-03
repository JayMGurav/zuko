use crate::config::zuko_context::ZukoContext;
use crate::db::zuko_cli::{get_all_questions, get_all_topics};

use crate::types::{Topic, AppState};
use crate::ui::run_ui;

pub async fn execute(
    _context: &ZukoContext,
    topic_slug: Option<String>,
    difficulty: Option<String>,
    _solved: bool,
) {
    let questions = match get_all_questions(topic_slug, difficulty).await {
        Result::Ok(qs) => qs,
        Result::Err(e) => {
            eprintln!("Failed to get questions from the database: {}", e);
            return;
        }
    };

    if questions.is_empty() {
        eprintln!("No questions found");
        return;
    }

    let mut topics = match get_all_topics().await {
        Result::Ok(ts) => ts,
        Result::Err(e) => {
            eprintln!("Failed to get topics from the database: {}", e);
            return;
        }
    };

    let new_topic = Topic {
        name: "All Topics".to_string(),
        slug: "".to_string(),
    };

    topics.insert(0, new_topic);


    let mut app_state = AppState::new(questions, topics);

    if let Err(e) = run_ui(&mut app_state) {
        eprintln!("Failed to run UI: {}", e);
    }
}
