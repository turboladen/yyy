use std::{fs::File, path::PathBuf};

use anyhow::bail;
pub use clap::Parser;
use clap::{Args, Subcommand};

use crate::web::brands::{IndexBrand, SeedBrand};

#[derive(Parser, Debug, Clone)]
#[command(author, version, about, long_about = None, propagate_version = true)]
pub struct Cli {
    #[command(subcommand)]
    command: Commands,
}

impl Cli {
    /// After parsing, this returns which command we got from the user.
    ///
    #[must_use]
    pub const fn command(&self) -> &Commands {
        &self.command
    }
}

/// With the help of `clap`'s `Subcommand`, this defines the subcommands we can pass to the app. Ex.
/// `cargo run validate` or `cargo run seed`.
///
#[derive(Subcommand, Debug, Clone)]
pub enum Commands {
    /// Boot the web app.
    ///
    Serve,

    /// Import data for the db from a file.
    ///
    ImportData(DataImporter),
}

#[derive(Args, Debug, Clone)]
pub struct DataImporter {
    /// The table to import to.
    ///
    table: String,

    /// The file that contains the data for the model.
    ///
    file: PathBuf,
}

impl DataImporter {
    pub async fn import(&self) -> anyhow::Result<()> {
        match self.table.as_str() {
            "brands" => {
                let seed_brands = {
                    let f = File::open(&self.file)?;
                    let data: Vec<SeedBrand> = serde_yaml::from_reader(f)?;
                    data
                };

                let db = crate::web::state::db().await?;

                let mut inserted: Vec<IndexBrand> = vec![];

                for seed_brand in seed_brands {
                    let i: IndexBrand = db
                        .create("brands")
                        .content(seed_brand.into_insert())
                        .await?;
                    inserted.push(i);
                }

                for brand in inserted {
                    println!("Inserted brand: [{}] {}", brand.id().id, brand.name());
                }

                Ok(())
            }
            t => {
                bail!("Unknown table type: {t}")
            }
        }
    }
}
