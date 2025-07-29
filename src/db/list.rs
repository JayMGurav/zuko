use crate::config::zuko_context::{ZukoContext};
use crate::db::get_zuko_db;
use crate::types::{Question};

pub async fn get_all_questions(
    _context: &ZukoContext,
    topic: Option<String>,
    difficulty: Option<String>,
)  -> Result<Vec<Question>, Box<dyn std::error::Error>>  {
    // Fetch questions from the database
    let db = get_zuko_db();

    // let mut query = "SELECT title,title_slug,topic,difficulty,content,hints,example_testcase_list,next_challenges FROM QuestionList".to_string();

    let mut query = "SELECT title,title_slug,content FROM QuestionList".to_string();

    let mut conditions = vec![];

    if let Some(ref t) = topic {
        conditions.push(format!("topic = '{}'", t));
    }

    if let Some(ref d) = difficulty {
        conditions.push(format!("difficulty = '{}'", d));
    }
    //TODO: Add solved status filter if needed
    
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
                    let question = Question::from_row(row)?;
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