use crate::db::get_zuko_db;
use crate::types::{Question,Topic};
use libsql::{de, params};

/// Sanitizes a slug string to only allow alphanumeric characters, hyphens, and underscores.
/// This prevents SQL injection when slugs are interpolated into queries.
fn sanitize_slug(input: &str) -> String {
    input
        .chars()
        .filter(|c| c.is_alphanumeric() || *c == '-' || *c == '_')
        .collect()
}

/// Validates that a difficulty string is one of the known safe values (case-insensitive).
/// Returns the canonical DB representation (e.g. "Easy") or `None` if unrecognised.
fn validate_difficulty(input: &str) -> Option<&'static str> {
    match input.to_lowercase().as_str() {
        "easy" => Some("Easy"),
        "medium" => Some("Medium"),
        "hard" => Some("Hard"),
        _ => None,
    }
}

const BASE_QUERY: &str = "SELECT title, title_slug, content, difficulty, topic, hints, example_testcase_list, similar_question_list, next_challenges FROM QuestionList";

/// JSON existence condition for topic filtering; uses `?1` as the placeholder for the slug.
const TOPIC_CONDITION: &str =
    "EXISTS (SELECT 1 FROM json_each(QuestionList.topic) WHERE json_each.value ->> '$.slug' = ?1)";

pub async fn get_all_questions(
    topic_slug: Option<String>,
    difficulty: Option<String>,
)  -> Result<Vec<Question>, Box<dyn std::error::Error>>  {

    let db = get_zuko_db();

    let safe_slug = topic_slug
        .as_deref()
        .filter(|s| !s.is_empty())
        .map(sanitize_slug);

    let safe_difficulty = difficulty
        .as_deref()
        .filter(|d| d.to_lowercase() != "all")
        .and_then(validate_difficulty);

    // Select the appropriate query variant and bind parameters to avoid SQL injection.
    let mut rows = match (safe_slug.as_deref(), safe_difficulty) {
        (Some(slug), Some(diff)) => {
            let q = format!("{BASE_QUERY} WHERE {TOPIC_CONDITION} AND difficulty = ?2");
            db.query(&q, params![slug, diff]).await?
        }
        (Some(slug), None) => {
            let q = format!("{BASE_QUERY} WHERE {TOPIC_CONDITION}");
            db.query(&q, params![slug]).await?
        }
        (None, Some(diff)) => {
            db.query(&format!("{BASE_QUERY} WHERE difficulty = ?1"), params![diff]).await?
        }
        (None, None) => {
            db.query(BASE_QUERY, ()).await?
        }
    };

    let mut questions = Vec::new();
    while let Some(row) = rows.next().await? {
        questions.push(de::from_row::<Question>(&row)?);
    }
    Ok(questions)

}

#[allow(dead_code)]
pub async fn get_question(title_slug: String) -> Result<Question, Box<dyn std::error::Error>> {
    let db = get_zuko_db();

    let mut stmt = db.prepare("SELECT * FROM QuestionList WHERE title_slug = $1").await.unwrap();


    let row = stmt.query([title_slug]) .await
        .unwrap()
        .next()
        .await
        .unwrap()
        .unwrap();

    let question = de::from_row::<Question>(&row).unwrap();

    Ok(question)
}


pub async fn get_all_topics() -> Result<Vec<Topic>, Box<dyn std::error::Error>> {
    let db = get_zuko_db();

    let query = "SELECT * FROM Topics".to_string();

    match db.query(&query, ()).await {
        Ok(mut rows) => {
            let mut topics = Vec::new();
            while let Some(row) = rows.next().await? {
                let topic: Topic = de::from_row(&row)?;
                topics.push(topic);
            }
            Ok(topics)
        }
        Err(e) => Err(format!("Failed to fetch topics: {}", e).into()),
    }
}