pub mod list;
use crate::types::Question;
use crate::utils::fuzzy_matcher::search_questions;

pub struct AppState {
    pub all_questions: Vec<Question>,
    pub filtered_question_indices: Vec<usize>,
    pub query: String,
    pub selected_index: usize,
    pub scroll: u16,
}

pub fn update_filtered(app: &mut AppState) {
    app.filtered_question_indices = search_questions(&app.all_questions, &app.query);
    if app.selected_index >= app.filtered_question_indices.len() {
        app.selected_index = 0;
    }
}