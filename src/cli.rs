//!
//! This module defines and handles logic for processing command-line arguments.
//!
pub(crate) mod import;

use clap::{Parser, Subcommand};

use self::import::Importer;

/// This struct enables the `clap::Parser` functionality, which does all the CLI argument parsing.
///
#[derive(Parser, Debug, Clone)]
#[command(author, version, about, long_about = None, propagate_version = true)]
pub(crate) struct Cli {
    #[command(subcommand)]
    command: Commands,
}

impl Cli {
    /// After parsing, this returns which command we got from the user.
    ///
    #[must_use]
    pub(crate) const fn command(&self) -> &Commands {
        &self.command
    }
}

/// With the help of `clap`'s `Subcommand`, this defines the subcommands we can pass to the app. Ex.
/// `cargo run validate` or `cargo run seed`.
///
#[derive(Subcommand, Debug, Clone)]
pub(crate) enum Commands {
    /// Boot the web app.
    ///
    Serve,

    /// Import data for the db from a file.
    ///
    Import(Importer),

    /// Create the database.
    ///
    DbCreate,
}
