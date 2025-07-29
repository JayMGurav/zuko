use crate::config::zuko_context::ZukoContext;
use crate::db::list::get_all_questions;

use crate::ui::AppState;
use crate::ui::list::run_ui;

pub async fn execute(
    context: &ZukoContext,
    topic: Option<String>,
    difficulty: Option<String>,
    _solved: bool,
) {
    let questions = match get_all_questions(context, topic, difficulty).await {
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

    let app_state = AppState {
        all_questions: questions,
        filtered_question_indices: Vec::new(),
        query: String::new(),
        selected_index: 0,
        scroll: 0,
    };

    if let Err(e) = run_ui(app_state) {
        eprintln!("Failed to run UI: {}", e);
    }
}
