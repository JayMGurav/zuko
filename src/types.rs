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

#[derive(Debug, Serialize, Deserialize)]
// #[serde(rename_all = "camelCase")]
pub struct Topic {
    pub name: String,
    pub slug: String,
}

#[derive(Debug, Serialize, Deserialize)]
// #[serde(rename_all = "camelCase")]
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

// ---- implementations ----

impl Question {
    // pub fn from_row(row: libsql::Row) -> Result<Self, Box<dyn std::error::Error>> {
    // Ok(Question {
    //     title: row.get(0)?,
    //     title_slug: row.get(1)?,
    //     content: row.get(2)?,

    //     // difficulty: serde_json::from_str(&row.get::<String>(3)?)?, 
        
    //     // topic: parse_optional_json(row.get::<Option<String>>(2)?)?,
    //     // hints: parse_optional_json(row.get::<Option<String>>(5)?)?,
    //     // example_testcase_list: parse_optional_json(row.get::<Option<String>>(6)?)?,
    //     // similar_question_list: parse_optional_json(row.get::<Option<String>>(7)?)?,
    //     // next_challenges: parse_optional_json(row.get::<Option<String>>(8)?)?,
    // })
// }
}

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
