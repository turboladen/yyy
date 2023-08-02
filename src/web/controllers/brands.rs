use axum::extract::State;
use maud::{html, Markup};

use crate::web::{error::Error, html::page, models::brands::IndexBrand, state::AppState};

#[axum_macros::debug_handler]
pub(crate) async fn index(State(state): State<AppState>) -> Result<Markup, Error> {
    let db = state.db.lock().await;
    let mut brands: Vec<IndexBrand> = db.select("brands").await?;
    brands.sort_by(|a, b| a.name().cmp(b.name()));

    Ok(page(
        "Brands",
        html! {
            div {
                table {
                    tr {
                        th { "ID" }
                        th { "Name" }
                    }
                    @for brand in &brands {
                        tr { td {  (brand.id().id) } td {  (brand.name()) } }
                    }
                }
            }
        },
    ))
}
