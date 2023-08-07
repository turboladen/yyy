use axum::response::{IntoResponse, Response};
use hyper::StatusCode;
use thiserror::Error;

/// Errors for web-specific stuff.
///
// TODO: If this is ever deployed, these should obfuscate implementation.
//
#[derive(Error, Debug)]
pub(crate) enum Error {
    /// Errors that happen while accessing the database.
    ///
    #[error(transparent)]
    Db(#[from] surrealdb::Error),

    /// Low-level HTTP errors.
    ///
    #[error(transparent)]
    Hyper(#[from] hyper::Error),
}

impl IntoResponse for Error {
    fn into_response(self) -> Response {
        // TODO: This should better handle errors, accordingly (i.e. maybe not all errors are 500s)
        (StatusCode::INTERNAL_SERVER_ERROR, self.to_string()).into_response()
    }
}
