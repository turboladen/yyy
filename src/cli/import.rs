use anyhow::bail;
use async_trait::async_trait;
use clap::Args;
use serde::de::DeserializeOwned;
use std::path::{Path, PathBuf};
use surrealdb::{engine::local::Db, Surreal};
use tokio::fs::File;

use crate::web::models::brands::SeedBrand;

/// This command allows for importing YAML data for one of our tables.
///
#[derive(Args, Debug, Clone)]
pub(crate) struct Importer {
    /// The table to import to.
    ///
    table: String,

    /// The file that contains the data for the model.
    ///
    file: PathBuf,
}

impl Importer {
    /// Call this to import the data to the given table.
    ///
    /// # Errors
    ///
    /// This will error if:
    ///
    /// 1. The given table name doesn't exist
    /// 2. If there was a problem opening the related file
    /// 3. If the provided file doesn't contain valid YAML
    /// 4. If inserted the data to the database fails
    ///
    pub(crate) async fn import(&self, db: &Surreal<Db>) -> anyhow::Result<()> {
        match self.table.as_str() {
            "brands" => SeedBrand::import(&self.file, db).await,
            t => {
                bail!("Unknown table type: {t}")
            }
        }
    }
}

/// This trait tries to make it easier for defining the specifics for importing data for a specific
/// table.
///
#[async_trait]
pub(crate) trait Import: DeserializeOwned {
    type InsertedType;

    async fn import(file: &Path, db: &Surreal<Db>) -> anyhow::Result<()>;

    async fn load_yaml(file: &Path) -> anyhow::Result<Vec<Self>> {
        let f = File::open(file).await?.into_std().await;
        let data: Vec<Self> = serde_yaml::from_reader(f)?;

        Ok(data)
    }
}
