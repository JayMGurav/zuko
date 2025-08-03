use crate::types::{Question, Topic};

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


pub fn search_topics(topics: &[Topic], query: &str) -> Vec<usize> {
    let mut matcher = Matcher::new(Config::DEFAULT);
    let pattern = Pattern::parse(query, CaseMatching::Ignore, Normalization::Smart);

    // Build a vector of topics with their indices
    let topic_refs: Vec<(usize, &str)> = topics
        .iter()
        .enumerate()
        .map(|(i, t)| (i, t.name.as_str()))
        .collect();

    let matches = pattern.match_list(topic_refs.iter().map(|(_, topic)| *topic), &mut matcher);

    matches
        .into_iter()
        .filter_map(|(topic, _score)| {
            topic_refs.iter().find(|(_, t)| *t == topic).map(|(i, _)| *i)
        })
        .collect()
}

