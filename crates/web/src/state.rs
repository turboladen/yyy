use std::sync::Arc;

use surrealdb::{
    engine::local::{Db, File},
    Surreal,
};
use tokio::sync::Mutex;

#[derive(Clone)]
pub(crate) struct AppState {
    pub(crate) db: Arc<Mutex<Surreal<Db>>>,
}

impl AppState {
    pub async fn try_new(db_file: &str) -> surrealdb::Result<Self> {
        let db = Surreal::new::<File>(db_file).await?;

        db.use_ns("yyy").use_db("dev").await?;

        Ok(Self {
            db: Arc::new(Mutex::new(db)),
        })
    }
}
