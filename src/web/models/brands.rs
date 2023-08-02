use std::path::Path;

use async_trait::async_trait;
use serde::{Deserialize, Serialize};
use surrealdb::{
    engine::local::Db,
    sql::{Datetime, Thing},
    Surreal,
};
use time::OffsetDateTime;
use tracing::info;

use crate::{cli::import::Import, database::CreateTable};

pub(crate) struct Creator;

impl CreateTable for Creator {
    const QUERY: &'static str = r#"
    DEFINE TABLE brands SCHEMAFULL;

    DEFINE FIELD name ON TABLE brands TYPE string
        ASSERT $value != NONE;

    DEFINE FIELD created_at ON TABLE brands TYPE datetime
        VALUE time::now();

    "#;
}

#[derive(Debug, Deserialize)]
pub(crate) struct IndexBrand {
    id: Thing,
    name: String,
    created_at: Datetime,
}

impl IndexBrand {
    pub(crate) const fn id(&self) -> &Thing {
        &self.id
    }

    pub(crate) fn name(&self) -> &str {
        self.name.as_ref()
    }
}

#[derive(Debug, Deserialize)]
pub(crate) struct SeedBrand {
    name: String,
}

impl SeedBrand {
    pub(crate) fn into_insert(self) -> InsertBrand {
        InsertBrand {
            name: self.name,
            created_at: OffsetDateTime::now_utc(),
        }
    }
}

#[async_trait]
impl Import for SeedBrand {
    type InsertedType = IndexBrand;

    async fn import(file: &Path, db: &Surreal<Db>) -> anyhow::Result<()> {
        let seed_brands = Self::load_yaml(file).await?;

        for seed_brand in seed_brands {
            info!("Creating brand: {:?}", &seed_brand);

            let brand: IndexBrand = db
                .create("brands")
                .content(seed_brand.into_insert())
                .await?;
            println!("Inserted brand: [{}] {}", brand.id().id, brand.name());
        }

        Ok(())
    }
}

#[derive(Debug, Serialize)]
pub(crate) struct InsertBrand {
    name: String,
    created_at: OffsetDateTime,
}
