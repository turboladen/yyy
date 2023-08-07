//!
//! This module handles getting/putting `brands` from/to the database.
//!
use std::path::Path;

use async_trait::async_trait;
use serde::{Deserialize, Serialize};
use surrealdb::{
    engine::local::Db,
    sql::{Datetime, Thing},
    Surreal,
};
use tracing::info;

use crate::{cli::import::Import, database::CreateTable};

/// This struct is solely for implementing the `CreateTable` trait.
///
pub(crate) struct Creator;

impl CreateTable for Creator {
    const QUERY: &'static str = r#"
    DEFINE TABLE brands SCHEMAFULL;

    DEFINE FIELD name ON TABLE brands TYPE string
        ASSERT $value != NONE;

    DEFINE FIELD created_at ON TABLE brands TYPE datetime
        VALUE $value OR time::now();
    "#;
}

/// Data for `/brands`.
///
#[derive(Debug, Deserialize)]
pub(crate) struct IndexBrand {
    id: Thing,
    name: String,
}

impl IndexBrand {
    pub(crate) const fn id(&self) -> &Thing {
        &self.id
    }

    pub(crate) fn name(&self) -> &str {
        self.name.as_ref()
    }
}

/// Data for `/brands/:id`.
///
#[derive(Debug, Deserialize)]
pub(crate) struct ShowBrand {
    id: Thing,
    name: String,
    created_at: Datetime,
}

impl ShowBrand {
    pub(crate) const fn id(&self) -> &Thing {
        &self.id
    }

    pub(crate) fn name(&self) -> &str {
        self.name.as_ref()
    }

    pub(crate) const fn created_at(&self) -> &chrono::DateTime<chrono::Utc> {
        &self.created_at.0
    }
}

/// Data for reading brands from the seed file and writing to the database. The seed file
/// only contains entries with names, hence the single attribute here.
///
#[derive(Debug, Deserialize, Serialize)]
pub(crate) struct SeedBrand {
    name: String,
}

#[async_trait]
impl Import for SeedBrand {
    type InsertedType = IndexBrand;

    async fn import(file: &Path, db: &Surreal<Db>) -> anyhow::Result<()> {
        let seed_brands = Self::load_yaml(file).await?;

        for seed_brand in seed_brands {
            info!("Creating brand: {:?}", &seed_brand);

            let brand: ShowBrand = db.create("brands").content(seed_brand).await?;
            println!(
                "Inserted brand: [{} - {}] {}",
                brand.created_at(),
                brand.id().id,
                brand.name()
            );
        }

        Ok(())
    }
}
