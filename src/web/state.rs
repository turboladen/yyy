//!
//! Specifically for the web app runtime, this wraps normal database access (and potentially any
//! other related state) in a type that's conducive to asynchronous access.
//!
use std::sync::Arc;

use surrealdb::{engine::local::Db, Surreal};
use tokio::sync::Mutex;
use tracing::debug_span;

use crate::{database, settings::Database};

/// All state is managed here.
///
#[derive(Clone)]
pub(crate) struct AppState {
    pub(crate) db: Arc<Mutex<Surreal<Db>>>,
}

impl AppState {
    /// Instantiate all the things.
    ///
    pub(crate) async fn try_new(db_settings: &Database) -> surrealdb::Result<Self> {
        let db = {
            debug_span!(
                "DB Setup",
                file = db_settings.file(),
                namespace = db_settings.namespace(),
                db = db_settings.name(),
            );

            database::connect(db_settings).await?
        };

        Ok(Self {
            db: Arc::new(Mutex::new(db)),
        })
    }
}
