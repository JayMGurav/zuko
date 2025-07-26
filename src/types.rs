use serde::{Deserialize, Serialize};
use clap::ValueEnum;
use std::fmt;

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