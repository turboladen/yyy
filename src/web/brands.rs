use axum::extract::State;
use maud::{html, Markup};
use serde::{Deserialize, Serialize};
use surrealdb::sql::Thing;
use time::OffsetDateTime;

use super::{error::Error, state::AppState};

#[derive(Debug, Serialize, Deserialize)]
pub struct IndexBrand {
    id: Thing,
    name: String,
    created_at: OffsetDateTime,
}

impl IndexBrand {
    pub fn id(&self) -> &Thing {
        &self.id
    }

    pub fn name(&self) -> &str {
        self.name.as_ref()
    }
}

#[axum_macros::debug_handler]
pub(crate) async fn index(State(state): State<AppState>) -> Result<Markup, Error> {
    let db = state.db.lock().await;
    let mut brands: Vec<IndexBrand> = db.select("brands").await?;
    brands.sort_by(|a, b| a.name.cmp(&b.name));

    Ok(html! {
        h1 { "Brands" }
        table {
            tr {
                th { "ID" }
                th { "Name" }
            }
            @for brand in &brands {
                tr { td {  (brand.id.id) } td {  (brand.name) } }
            }
        }
    })
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SeedBrand {
    name: String,
}

impl SeedBrand {
    pub fn into_insert(self) -> InsertBrand {
        InsertBrand {
            name: self.name,
            created_at: OffsetDateTime::now_utc(),
        }
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct InsertBrand {
    name: String,
    created_at: OffsetDateTime,
}
