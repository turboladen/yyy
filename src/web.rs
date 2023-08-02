pub(crate) mod controllers;
pub mod error;
pub(crate) mod html;
pub(crate) mod models;
pub(crate) mod state;

use axum::{routing::get, Router};
use tower::ServiceBuilder;
use tower_http::trace::TraceLayer;
use tracing::info;

pub use self::error::Error;

use self::state::AppState;

#[tracing::instrument]
pub async fn start() -> Result<(), self::Error> {
    let state = AppState::try_new().await?;

    let app = Router::new()
        .route("/", get(|| async { "Hello, World!" }))
        .route("/brands", get(controllers::brands::index))
        .layer(ServiceBuilder::new().layer(TraceLayer::new_for_http()))
        .with_state(state);

    let host_and_port = "0.0.0.0:3000";

    info!("Starting server on {host_and_port}...");

    axum::Server::bind(&host_and_port.parse().unwrap())
        .serve(app.into_make_service())
        .await?;

    Ok(())
}
