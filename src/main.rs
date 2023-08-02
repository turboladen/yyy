pub(crate) mod cli;
pub(crate) mod database;
pub(crate) mod web;

use crate::cli::{Cli, Commands, Parser};

use self::database::DbForCreate;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    env_logger::init();
    let args = Cli::parse();

    match args.command() {
        Commands::Serve => Ok(web::start().await?),
        Commands::Import(data_importer) => Ok(data_importer.import().await?),
        Commands::DbCreate => {
            let db = DbForCreate::try_new().await?;

            Ok(db.create().await?)
        }
    }
}
