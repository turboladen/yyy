pub(crate) mod brands;
pub mod error;
pub(crate) mod state;

use axum::{routing::get, Router};

pub use self::error::Error;

use self::state::AppState;

#[tracing::instrument]
pub async fn start() -> Result<(), self::Error> {
    tracing::info!("meow");
    let state = AppState::try_new("yyy.dev.db").await?;

    let app = Router::new()
        .route("/", get(|| async { "Hello, World!" }))
        .route("/brands", get(brands::index))
        .with_state(state);

    axum::Server::bind(&"0.0.0.0:3000".parse().unwrap())
        .serve(app.into_make_service())
        .await
        .unwrap();

    Ok(())
}
