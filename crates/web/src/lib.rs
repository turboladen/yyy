pub(crate) mod brands;
pub mod error;
pub(crate) mod state;

use axum::{routing::get, Router};
use tracing::info;

pub use self::error::Error;

use self::state::AppState;

#[tracing::instrument]
pub async fn start() -> Result<(), self::Error> {
    let state = AppState::try_new("yyy.dev.db").await?;

    let app = Router::new()
        .route("/", get(|| async { "Hello, World!" }))
        .route("/brands", get(brands::index))
        .with_state(state);

    let host_and_port = "0.0.0.0:3000";

    info!("Starting server on {host_and_port}...");

    axum::Server::bind(&host_and_port.parse().unwrap())
        .serve(app.into_make_service())
        .await?;

    Ok(())
}
