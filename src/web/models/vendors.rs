//!
//! This module handles getting/putting `vendors` from/to the database.
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
    DEFINE TABLE vendors SCHEMAFULL;

    DEFINE FIELD name ON TABLE vendors TYPE string
        ASSERT $value != NONE;

    DEFINE FIELD home_page ON TABLE vendors TYPE string
        ASSERT $value != NONE
            AND is::url($value);

    DEFINE FIELD created_at ON TABLE vendors TYPE datetime
        VALUE $value OR time::now();
    "#;
}

/// Data for `/vendors`.
///
#[derive(Debug, Deserialize)]
pub(crate) struct Index {
    id: Thing,
    name: String,

    // This *could* be a URI type, but we don't need any of that functionality at this time.
    //
    home_page: String,
}

impl Index {
    pub(crate) const fn id(&self) -> &Thing {
        &self.id
    }

    pub(crate) fn name(&self) -> &str {
        self.name.as_ref()
    }

    pub(crate) fn home_page(&self) -> &str {
        self.home_page.as_ref()
    }
}

/// Data for `/vendors/:id`.
///
#[derive(Debug, Deserialize)]
pub(crate) struct Show {
    id: Thing,
    name: String,
    home_page: String,
    created_at: Datetime,
}

impl Show {
    pub(crate) const fn id(&self) -> &Thing {
        &self.id
    }

    pub(crate) fn name(&self) -> &str {
        self.name.as_ref()
    }

    pub(crate) const fn created_at(&self) -> &chrono::DateTime<chrono::Utc> {
        &self.created_at.0
    }

    pub(crate) fn home_page(&self) -> &str {
        self.home_page.as_ref()
    }
}

/// Data for reading vendors from the seed file and writing to the database. The seed file
/// only contains entries with names, hence the single attribute here.
///
#[derive(Debug, Deserialize, Serialize)]
pub(crate) struct Seed {
    id: u16,
    name: String,
    home_page: String,
}

#[async_trait]
impl Import for Seed {
    type InsertedType = Index;

    async fn import(file: &Path, db: &Surreal<Db>) -> anyhow::Result<()> {
        let seed_vendors = Self::load_yaml(file).await?;

        for seed_vendor in seed_vendors {
            info!("Creating vendor: {:?}", &seed_vendor);

            let vendor: Show = db.create("vendors").content(seed_vendor).await?;
            println!(
                "Inserted vendor: [{} - {}] {}: {}",
                vendor.created_at(),
                vendor.id().id,
                vendor.name(),
                vendor.home_page()
            );
        }

        Ok(())
    }
}
