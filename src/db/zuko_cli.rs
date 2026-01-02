use crate::db::get_zuko_db;
use crate::types::{Question,Topic};
use libsql::{de};



pub async fn get_all_questions(
    topic_slug: Option<String>,
    difficulty: Option<String>,
)  -> Result<Vec<Question>, Box<dyn std::error::Error>>  {

    // Fetch questions from the database
    let db = get_zuko_db();
    
        // let mut query = "SELECT title,title_slug,content FROM QuestionList".to_string();

    let mut query = "SELECT title, title_slug, content, difficulty, topic, hints, example_testcase_list, similar_question_list, next_challenges FROM QuestionList".to_string();

    let mut conditions = vec![];

    if let Some(ref topic_slug) = topic_slug {
        if !topic_slug.is_empty() {
            conditions.push(format!(
                "EXISTS (SELECT 1 FROM json_each(QuestionList.topic) WHERE json_each.value ->> '$.slug' = '{}')",
                topic_slug
            ));
        }
    }


    if let Some(ref d) = difficulty {
        if d != "ALL" {
            conditions.push(format!("difficulty = '{}'", d));
        }
    }
    
    if !conditions.is_empty() {
        query.push_str(" WHERE ");
        query.push_str(&conditions.join(" AND "));
    }


    match db.query(&query, ()).await {
    Ok(mut rows) => {
        let mut questions = Vec::new();
        loop {
            match rows.next().await? {
                Some(row) => {
                    // println!("Row values: {:?}", row);
                    // let question = Question::from_row(row)?;
                    let question: Question = de::from_row(&row)?;
                    questions.push(question);
                }
                None => break,
            }
        }
        Ok(questions)
    }
    Err(e) => Err(format!("Failed to fetch questions: {}", e).into()),
}

}

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
            loop {
                match rows.next().await? {
                    Some(row) => {
                        // println!("Row values: {:?}", row);
                        // let question = Question::from_row(row)?;
                        let topic: Topic = de::from_row(&row)?;
                        topics.push(topic);
                    }
                    None => break,
                }
            }
            Ok(topics)
        }
        Err(e) => Err(format!("Failed to fetch topics: {}", e).into()),
    }
}