#![deny(unused_extern_crates)]
#![warn(
    clippy::all,
    clippy::nursery,
    clippy::pedantic,
    future_incompatible,
    missing_copy_implementations,
    missing_docs,
    nonstandard_style,
    rust_2018_idioms,
    trivial_casts,
    trivial_numeric_casts,
    unreachable_pub,
    unused_qualifications
)]
#![allow(clippy::redundant_pub_crate)]

pub(crate) mod cli;
pub(crate) mod database;
pub(crate) mod web;

use clap::Parser;

use crate::cli::{Cli, Commands};

use self::database::DbForCreate;

/// While the app is obviously primarily a web app, we define some other commands here that help
/// manage the app.
///
#[tokio::main]
async fn main() -> anyhow::Result<()> {
    env_logger::init();
    let args = Cli::parse();

    match args.command() {
        Commands::Serve => Ok(web::start().await?),
        Commands::Import(data_importer) => {
            let db = database::connect().await?;

            Ok(data_importer.import(&db).await?)
        }
        Commands::DbCreate => {
            let db = DbForCreate::try_new().await?;

            Ok(db.create().await?)
        }
    }
}
