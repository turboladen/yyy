use axum::async_trait;
use surrealdb::{
    engine::local::{Db, File},
    method::Query,
    Surreal,
};
use tracing::info;

pub(crate) const DB_FILE: &str = "yyy.dev.db";
pub(crate) const NAMESPACE: &str = "yyy";
pub(crate) const DEV_DB_NAME: &str = "dev";

pub(crate) async fn connect() -> surrealdb::Result<Surreal<Db>> {
    let db = Surreal::new::<File>(DB_FILE).await?;
    db.use_ns(NAMESPACE).use_db(DEV_DB_NAME).await?;

    Ok(db)
}

pub(crate) struct DbForCreate {
    db: Surreal<Db>,
}

impl DbForCreate {
    pub(crate) async fn try_new() -> surrealdb::Result<Self> {
        Ok(Self {
            db: connect().await?,
        })
    }

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

#[async_trait]
pub(crate) trait CreateTable {
    const QUERY: &'static str;

    async fn create_table(query: Query<'_, Db>) -> Query<'_, Db> {
        query.query(Self::QUERY)
    }
}
