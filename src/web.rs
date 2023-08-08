//!
//! All code specific to the web app lives here.
//!
pub(crate) mod controllers;
pub(crate) mod error;
pub(crate) mod models;
pub(crate) mod state;
pub(crate) mod views;

use std::time::Duration;

use axum::{routing::get, Router};
use tower::ServiceBuilder;
use tower_http::{compression::CompressionLayer, timeout::TimeoutLayer, trace::TraceLayer};
use tracing::info;

use crate::settings::Settings;

use self::state::AppState;

/// This starts the app. Normally you'd find this in a `main()` function, but our app does more than
/// launch the web app, hence a regular ol' function.
///
#[tracing::instrument]
pub(crate) async fn start(settings: &Settings) -> anyhow::Result<()> {
    let state = AppState::try_new(settings.database()).await?;

    let app = Router::new()
        .route("/", get(|| async { "Hello, World!" }))
        .route("/brands", get(controllers::brands::index))
        .route("/vendors", get(controllers::vendors::index))
        .route(
            "/project_archetypes",
            get(controllers::project_archetypes::index),
        )
        .layer(
            ServiceBuilder::new()
                .layer(CompressionLayer::new())
                .layer(TraceLayer::new_for_http())
                .layer(TimeoutLayer::new(Duration::from_secs(30))),
        )
        .with_state(state);

    let addr = settings.tcp().socket_addr()?;
    info!("Starting server on {addr}...");

    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await?;

    Ok(())
}
