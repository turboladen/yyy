//!
//! All HTTP endpoints for `projects`.
//!
use axum::extract::State;
use maud::Markup;

use crate::web::{
    error::Error, models::projects::Index, state::AppState, views::projects::index_html,
};

/// (HTML) `GET /projects`
///
#[axum_macros::debug_handler]
pub(crate) async fn index(State(state): State<AppState>) -> Result<Markup, Error> {
    const QUERY: &str = "SELECT * FROM projects ORDER BY name;";

    // Scoping here so the DB lock can get dropped sooner than later.
    let response = {
        let db = state.db.lock().await;
        db.query(QUERY).await?
    };

    let projects: Vec<Index> = {
        let mut response = response.check()?;
        response.take(0)?
    };

    Ok(index_html(&projects))
}
