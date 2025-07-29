// use libsql::{Builder, Connection};
// use crate::db::{ZUKO_USER_DB};

// pub async fn connect_to_zuko_user_db(db_path: String) -> Result<Connection, String> {

//     let zuko_user_db = Builder::new_local(
//             db_path,
//         )
//         .build()
//         .await
//         .map_err(|e| format!("Failed to build connection to zuko_user.db: {}", e))?;

//         let zuko_user_db_connection = zuko_user_db.connect().map_err(|e| format!("Failed to connect to zuko_user.db: {}", e))?;

//         // get or initialize the ZUKO_USER_DB
//         ZUKO_USER_DB
//             .set(zuko_user_db_connection.clone())
//             .expect("ZUKO_USER_DB already initialized!");

//         Ok(zuko_user_db_connection)
// }


// pub async fn initialize_zuko_user_db(db_path: String) {

//     let zuko_user_db_connection = connect_to_zuko_user_db(db_path).await.expect("Failed to connect to zuko_user.db");

//     // Create the user table if it doesn't exist
// }






// "CREATE TABLE IF NOT EXISTS users (
//             id INTEGER PRIMARY KEY AUTOINCREMENT,
//             username TEXT NOT NULL UNIQUE,
//             project_name TEXT NOT NULL,
//             editor_cmd TEXT NOT NULL,
//             preferred_language_list TEXT NOT NULL,
//             difficulty TEXT NOT NULL,
//             init_git BOOLEAN NOT NULL DEFAULT false,
//             track_progress BOOLEAN NOT NULL DEFAULT false
//         )"