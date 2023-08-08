//!
//! All HTTP endpoints for `project_archetypes`.
//!
use axum::extract::State;
use maud::Markup;

use crate::web::{
    error::Error, models::project_archetypes::Index, state::AppState,
    views::project_archetypes::index_html,
};

/// (HTML) `GET /project_archetypes`
///
#[axum_macros::debug_handler]
pub(crate) async fn index(State(state): State<AppState>) -> Result<Markup, Error> {
    const QUERY: &str = "SELECT * FROM project_archetypes ORDER BY name;";

    // Scoping here so the DB lock can get dropped sooner than later.
    let response = {
        let db = state.db.lock().await;
        db.query(QUERY).await?
    };

    let project_archetypes: Vec<Index> = {
        let mut response = response.check()?;
        response.take(0)?
    };

    Ok(index_html(&project_archetypes))
}
