use crate::db::{sync_zuko_db};


pub async fn execute() {
    sync_zuko_db().await;
}