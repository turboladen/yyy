//!
//! All HTTP endpoints for `brands`.
//!
use axum::extract::State;
use maud::Markup;

use crate::web::{error::Error, models::brands::IndexBrand, state::AppState, views::brands::*};

/// (HTML) `GET /brands`
///
#[axum_macros::debug_handler]
pub(crate) async fn index(State(state): State<AppState>) -> Result<Markup, Error> {
    const QUERY: &str = "SELECT * FROM brands ORDER BY name;";

    // Scoping here so the DB lock can get dropped sooner than later.
    let response = {
        let db = state.db.lock().await;
        db.query(QUERY).await?
    };

    let brands: Vec<IndexBrand> = {
        let mut response = response.check()?;
        response.take(0)?
    };

    Ok(index_html(&brands))
}
