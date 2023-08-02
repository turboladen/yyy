use serde::{Deserialize, Serialize};
use surrealdb::sql::{Datetime, Thing};
use time::OffsetDateTime;

use crate::database::CreateTable;

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
    pub(crate) fn id(&self) -> &Thing {
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

#[derive(Debug, Serialize)]
pub(crate) struct InsertBrand {
    name: String,
    created_at: OffsetDateTime,
}
