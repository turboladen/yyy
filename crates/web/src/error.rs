use axum::response::{IntoResponse, Response};
use hyper::StatusCode;
use thiserror::Error;

// TODO: If this is ever deployed, these should obfuscate implementation.
//
#[derive(Error, Debug)]
pub enum Error {
    #[error(transparent)]
    Db(#[from] surrealdb::Error),

    #[error(transparent)]
    Hyper(#[from] hyper::Error),
}

impl IntoResponse for Error {
    fn into_response(self) -> Response {
        (StatusCode::INTERNAL_SERVER_ERROR, self.to_string()).into_response()
    }
}
