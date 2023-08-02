use anyhow::bail;
use async_trait::async_trait;
use clap::Args;
use serde::de::DeserializeOwned;
use std::path::{Path, PathBuf};
use surrealdb::{engine::local::Db, Surreal};
use tokio::fs::File;

use crate::web::models::brands::SeedBrand;

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
    pub(crate) async fn import(&self, db: &Surreal<Db>) -> anyhow::Result<()> {
        match self.table.as_str() {
            "brands" => SeedBrand::import(&self.file, db).await,
            t => {
                bail!("Unknown table type: {t}")
            }
        }
    }
}

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
