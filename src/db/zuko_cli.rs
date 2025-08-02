use crate::db::get_zuko_db;
use crate::config::zuko_context::{ZukoContext};
use crate::types::{Question};
use libsql::{de};


pub async fn get_question(context: &ZukoContext, title_slug: String) -> Result<Question, Box<dyn std::error::Error>> {
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
