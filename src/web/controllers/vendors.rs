//!
//! All HTTP endpoints for `vendors`.
//!
use axum::extract::State;
use maud::Markup;

use crate::web::{
    error::Error, models::vendors::Index, state::AppState, views::vendors::index_html,
};

/// (HTML) `GET /vendors`
///
#[axum_macros::debug_handler]
pub(crate) async fn index(State(state): State<AppState>) -> Result<Markup, Error> {
    const QUERY: &str = "SELECT * FROM vendors ORDER BY name;";

    // Scoping here so the DB lock can get dropped sooner than later.
    let response = {
        let db = state.db.lock().await;
        db.query(QUERY).await?
    };

    let vendors: Vec<Index> = {
        let mut response = response.check()?;
        response.take(0)?
    };

    Ok(index_html(&vendors))
}
