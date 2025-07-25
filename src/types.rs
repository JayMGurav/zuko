use serde::{Deserialize, Serialize};
use clap::ValueEnum;

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



// ----


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