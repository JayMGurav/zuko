use crate::types::Question;

use nucleo_matcher::{
    pattern::{Pattern, CaseMatching, Normalization},
    Matcher, Config,
};

pub fn search_questions(questions: &[Question], query: &str) -> Vec<usize> {
    let mut matcher = Matcher::new(Config::DEFAULT);
    let pattern = Pattern::parse(query, CaseMatching::Ignore, Normalization::Smart);

    // Build a vector of titles with their indices
    let title_refs: Vec<(usize, &str)> = questions
        .iter()
        .enumerate()
        .map(|(i, q)| (i, q.title.as_str()))
        .collect();
    
    let matches = pattern.match_list(title_refs.iter().map(|(_, title)| *title), &mut matcher);

    matches
        .into_iter()
        .filter_map(|(title, _score)| {
            title_refs.iter().find(|(_, t)| *t == title).map(|(i, _)| *i)
        })
        .collect()
}
