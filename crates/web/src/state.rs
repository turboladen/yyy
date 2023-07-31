use std::sync::Arc;

use surrealdb::{
    engine::local::{Db, File},
    Surreal,
};
use tokio::sync::Mutex;
use tracing::debug_span;

const NAMESPACE: &str = "yyy";
const DEV_DB_NAME: &str = "dev";

#[derive(Clone)]
pub(crate) struct AppState {
    pub(crate) db: Arc<Mutex<Surreal<Db>>>,
}

impl AppState {
    pub async fn try_new(db_file: &str) -> surrealdb::Result<Self> {
        let db = {
            debug_span!(
                "DB Setup",
                file = db_file,
                namespace = NAMESPACE,
                db = DEV_DB_NAME,
            );

            let db = Surreal::new::<File>(db_file).await?;
            db.use_ns(NAMESPACE).use_db(DEV_DB_NAME).await?;
            db
        };

        Ok(Self {
            db: Arc::new(Mutex::new(db)),
        })
    }
}
