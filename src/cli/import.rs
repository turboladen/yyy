use anyhow::bail;
use clap::Args;
use std::{fs::File, path::PathBuf};
use tracing::info;

use crate::web::models::brands::{IndexBrand, SeedBrand};

#[derive(Args, Debug, Clone)]
pub struct Importer {
    /// The table to import to.
    ///
    table: String,

    /// The file that contains the data for the model.
    ///
    file: PathBuf,
}

impl Importer {
    pub async fn import(&self) -> anyhow::Result<()> {
        match self.table.as_str() {
            "brands" => {
                let seed_brands = {
                    let f = File::open(&self.file)?;
                    let data: Vec<SeedBrand> = serde_yaml::from_reader(f)?;
                    data
                };

                let db = crate::database::connect_to_db().await?;

                let mut inserted: Vec<IndexBrand> = vec![];

                for seed_brand in seed_brands {
                    info!("Creating brand: {:?}", &seed_brand);

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
