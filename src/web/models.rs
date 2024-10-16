//! This module contains code for reading and writing data from database tables.
//!
pub(crate) mod brands;
pub(crate) mod capacitors;
pub(crate) mod project_archetypes;
pub(crate) mod vendors;

#[derive(Debug, serde::Deserialize, serde::Serialize)]
pub(crate) struct Measurement<T> {
    value: T,
    unit: String,
}
