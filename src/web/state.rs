use std::sync::Arc;

use surrealdb::{engine::local::Db, Surreal};
use tokio::sync::Mutex;
use tracing::debug_span;

use crate::database::{self, DB_FILE, DEV_DB_NAME, NAMESPACE};

#[derive(Clone)]
pub(crate) struct AppState {
    pub(crate) db: Arc<Mutex<Surreal<Db>>>,
}

impl AppState {
    pub(crate) async fn try_new() -> surrealdb::Result<Self> {
        let db = {
            debug_span!(
                "DB Setup",
                file = DB_FILE,
                namespace = NAMESPACE,
                db = DEV_DB_NAME,
            );

            database::connect().await?
        };

        Ok(Self {
            db: Arc::new(Mutex::new(db)),
        })
    }
}
