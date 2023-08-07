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

use crate::settings::Database;

/// Simple function for connecting to the database. Since the webapp needs a shareable connection,
/// this is really only useful for simple connections (i.e. for seeding data).
///
pub(crate) async fn connect(db_settings: &Database) -> surrealdb::Result<Surreal<Db>> {
    let db = Surreal::new::<File>(db_settings.file()).await?;

    db.use_ns(db_settings.namespace())
        .use_db(db_settings.name())
        .await?;

    Ok(db)
}

/// This wrapper for the Surreal database we use just adds some convenience methods for setting up
/// and working with the database from a maintenance perspective.
///
pub(crate) struct DbForMigrate {
    db: Surreal<Db>,
    name: String,
}

impl DbForMigrate {
    /// Connect to the db and instantiate `Self`.
    ///
    pub(crate) async fn try_new(db_settings: &Database) -> surrealdb::Result<Self> {
        Ok(Self {
            db: connect(db_settings).await?,
            name: db_settings.name().to_string(),
        })
    }

    /// This creates the database and sets up tables that we need. You can still use Surreal without
    /// doing any of this, but this gets us into the schema-full realm of Surreal instead of working
    /// schema-less (at this point, we want schema-full).
    ///
    pub(crate) async fn migrate(&self) -> surrealdb::Result<()> {
        info!("Creating database...");

        let response = self
            .db
            .query("BEGIN TRANSACTION")
            .query(format!("DEFINE DATABASE {}", &self.name))
            .query(crate::web::models::brands::Creator::QUERY)
            .query(crate::web::models::vendors::Creator::QUERY)
            .query(crate::web::models::projects::Creator::QUERY)
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
