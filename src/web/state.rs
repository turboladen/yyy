use std::sync::Arc;

use surrealdb::{
    engine::local::{Db, File},
    Surreal,
};
use tokio::sync::Mutex;
use tracing::debug_span;

const DB_FILE: &str = "yyy.dev.db";
const NAMESPACE: &str = "yyy";
const DEV_DB_NAME: &str = "dev";

#[derive(Clone)]
pub(crate) struct AppState {
    pub(crate) db: Arc<Mutex<Surreal<Db>>>,
}

impl AppState {
    pub async fn try_new() -> surrealdb::Result<Self> {
        let db = {
            debug_span!(
                "DB Setup",
                file = DB_FILE,
                namespace = NAMESPACE,
                db = DEV_DB_NAME,
            );

            db().await?
        };

        Ok(Self {
            db: Arc::new(Mutex::new(db)),
        })
    }
}

pub(crate) async fn db() -> surrealdb::Result<Surreal<Db>> {
    let db = Surreal::new::<File>(DB_FILE).await?;
    db.use_ns(NAMESPACE).use_db(DEV_DB_NAME).await?;

    Ok(db)
}
