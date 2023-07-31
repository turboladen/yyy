use axum::extract::State;
use maud::{html, Markup};
use serde::{Deserialize, Serialize};
use surrealdb::sql::Thing;

use crate::{error::Error, state::AppState};

#[derive(Debug, Serialize, Deserialize)]
pub struct IndexBrand {
    id: Thing,
    name: String,
}

#[axum_macros::debug_handler]
pub(crate) async fn index(State(state): State<AppState>) -> Result<Markup, Error> {
    let db = state.db.lock().await;
    let brands: Vec<IndexBrand> = db.select("brands").await?;

    Ok(html! {
        h1 { "Brands" }
        ol {
            @for brand in &brands {
                li { (brand.id) (brand.name) }
            }
        }
    })
}
