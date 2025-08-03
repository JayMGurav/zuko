use clap::ValueEnum;
use serde::{Deserialize, Serialize};
use std::fmt;
use crate::utils::serde_json_string;

#[derive(Debug, Serialize, Deserialize)]
// #[serde(rename_all = "camelCase")]
pub struct SimilarQuestion {
    pub title: String,
    pub titleSlug: String,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Topic {
    pub name: String,
    pub slug: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Question {
    pub title: String,
    pub title_slug: String,
    pub content: String,
    pub difficulty: Option<String>,

    #[serde(deserialize_with = "serde_json_string::deserialize")]
    pub topic: Option<Vec<Topic>>,
    
    #[serde(deserialize_with = "serde_json_string::deserialize")]
    pub hints: Option<Vec<String>>,

    #[serde(deserialize_with = "serde_json_string::deserialize")]
    pub example_testcase_list: Option<Vec<String>>,

    #[serde(deserialize_with = "serde_json_string::deserialize")]
    pub similar_question_list: Option<Vec<SimilarQuestion>>,
    
    #[serde(deserialize_with = "serde_json_string::deserialize")]
    pub next_challenges: Option<Vec<String>>,
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize, ValueEnum)]
pub enum Difficulty {
    Easy,
    Medium,
    Hard,
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub enum DifficultyFilter {
    Specific(Difficulty),
    All,
}

pub enum CurrentScreen {
    QuestionList,
    TopicList,
    DifficultyFilter,
}

pub struct AppState {
    pub all_questions: Vec<Question>,
    pub all_topics: Vec<Topic>,
    pub difficulties: Vec<DifficultyFilter>,
    pub filtered_topic_indices: Vec<usize>,
    pub selected_topic_index: usize,
    pub selected_topic: Option<Topic>,
    pub selected_difficulty: DifficultyFilter,
    pub filtered_question_indices: Vec<usize>,
    pub query: String,
    pub topic_query: String,
    pub selected_index: usize,
    pub scroll: u16,
    pub current_screen: CurrentScreen,
}

impl AppState {
    pub fn new(questions: Vec<Question>, topics: Vec<Topic>) -> Self {
        AppState {
            all_questions: questions,
            filtered_question_indices: Vec::new(),
            all_topics:topics,
            filtered_topic_indices:Vec::new(),
            difficulties: DifficultyFilter::all_difficulties(),
            selected_difficulty: DifficultyFilter::default(),
            query: String::new(),
            topic_query: String::new(),
            selected_index: 0,
            selected_topic_index: 0,
            selected_topic: None,
            scroll: 0,
            current_screen: CurrentScreen::QuestionList,
        }
    }
}

// ---- implementations ----


impl fmt::Display for Difficulty {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let label = match self {
            Difficulty::Easy => "Easy 🟢",
            Difficulty::Medium => "Medium 🟡",
            Difficulty::Hard => "Hard 🔴",
        };
        write!(f, "{}", label)
    }
}

impl fmt::Display for DifficultyFilter {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            DifficultyFilter::All => write!(f, "All Difficulties 🌈"),
            DifficultyFilter::Specific(d) => write!(f, "{d}"), // `d` already implements Display
        }
    }
}

impl DifficultyFilter {
    pub fn default() -> Self {
        DifficultyFilter::All
    }
    pub fn all_difficulties() -> Vec<DifficultyFilter> {
        vec![
            DifficultyFilter::All,
            DifficultyFilter::Specific(Difficulty::Easy),
            DifficultyFilter::Specific(Difficulty::Medium),
            DifficultyFilter::Specific(Difficulty::Hard),
        ]
    }
}

impl std::str::FromStr for DifficultyFilter {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_lowercase().as_str() {
            "easy" => Ok(DifficultyFilter::Specific(Difficulty::Easy)),
            "medium" => Ok(DifficultyFilter::Specific(Difficulty::Medium)),
            "hard" => Ok(DifficultyFilter::Specific(Difficulty::Hard)),
            "all" => Ok(DifficultyFilter::All),
            _ => Err(format!("Invalid difficulty: {}", s)),
        }
    }
}
