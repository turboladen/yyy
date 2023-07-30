use clap::{Parser, Subcommand};

#[derive(Parser, Debug, Clone, Copy)]
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
#[derive(Subcommand, Debug, Clone, Copy)]
pub enum Commands {
    /// Boot the web app.
    ///
    Serve,

    /// Import data for the db from a file.
    ///
    ImportData,
}

#[tokio::main]
async fn main() {
    let args = Cli::parse();

    match args.command() {
        Commands::Serve => yyy_web::start().await,
        Commands::ImportData => {
            todo!()
        }
    }
}
