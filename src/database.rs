//!
//! This module is specifically for configuring and connecting to a Surreal database.
//! Table-specific queries should be defined in their associated model files.
//!
use axum::async_trait;
use surrealdb::{
    engine::local::{Db, File},
    method::Query,
    Surreal,
};
use tracing::info;

/// Since we're using a file-based RocksDB backend for now, this ends up being the name of the
/// directory that's created when Surreal inits.
///
pub(crate) const DB_FILE: &str = "yyy.dev.db";

/// At this point, we don't really care about namespacing, but Surreal requires it, so here we go.
///
pub(crate) const NAMESPACE: &str = "yyy";

/// Same as with `NAMESPACE`, we don't need more than one database yet, but have to declare a name
/// to use.
///
// TODO: If it ever makes sense, this could/should reflect the environment we're running in.
//
pub(crate) const DEV_DB_NAME: &str = "dev";

/// Simple function for connecting to the database. Since the webapp needs a shareable connection,
/// this is really only useful for simple connections (i.e. for seeding data).
///
pub(crate) async fn connect() -> surrealdb::Result<Surreal<Db>> {
    let db = Surreal::new::<File>(DB_FILE).await?;
    db.use_ns(NAMESPACE).use_db(DEV_DB_NAME).await?;

    Ok(db)
}

/// This wrapper for the Surreal database we use just adds some convenience methods for setting up
/// and working with the database from a maintenance perspective.
///
pub(crate) struct DbForCreate {
    db: Surreal<Db>,
}

impl DbForCreate {
    /// Connect to the db and instantiate `Self`.
    ///
    pub(crate) async fn try_new() -> surrealdb::Result<Self> {
        Ok(Self {
            db: connect().await?,
        })
    }

    /// This creates the database and sets up tables that we need. You can still use Surreal without
    /// doing any of this, but this gets us into the schema-full realm of Surreal instead of working
    /// schema-less (at this point, we want schema-full).
    ///
    pub(crate) async fn create(&self) -> surrealdb::Result<()> {
        info!("Creating database...");

        let response = self
            .db
            .query("BEGIN TRANSACTION")
            .query(format!("DEFINE DATABASE {DEV_DB_NAME}"))
            .query(crate::web::models::brands::Creator::QUERY)
            .query("COMMIT TRANSACTION")
            .await?;

        response.check()?;

        Ok(())
    }
}

/// This just defines a standard interface for creating tables as part of one transaction.
///
#[async_trait]
pub(crate) trait CreateTable {
    const QUERY: &'static str;

    async fn create_table(query: Query<'_, Db>) -> Query<'_, Db> {
        query.query(Self::QUERY)
    }
}
