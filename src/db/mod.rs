pub mod zuko_cli;
pub mod zuko_user;
// pub mod list;

use libsql::{Connection, Database};
use tokio::sync::OnceCell;

pub static ZUKO_DATABASE: OnceCell<Database> = OnceCell::const_new(); 
pub static ZUKO_DB: OnceCell<Connection> = OnceCell::const_new();
// pub static ZUKO_USER_DB: OnceCell<Connection> = OnceCell::const_new();


pub fn get_zuko_db() -> Connection {
    ZUKO_DB.get().expect("ZUKO_DB not initialized!").clone()
}

// pub fn get_zuko_user_db() -> Connection {
//     ZUKO_USER_DB.get().expect("ZUKO_USER_DB not initialized!").clone()
// }

pub async fn sync_zuko_db() {
    let zuko_database = ZUKO_DATABASE.get().expect("ZUKO_DATABASE not initialized!");
    zuko_database.sync().await.expect("Failed to sync zuko DB");
}   