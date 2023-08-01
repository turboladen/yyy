use yyy::cli::{Cli, Commands, Parser};

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    env_logger::init();
    let args = Cli::parse();

    match args.command() {
        Commands::Serve => Ok(yyy::web::start().await?),
        Commands::ImportData(data_importer) => Ok(data_importer.import().await?),
    }
}
