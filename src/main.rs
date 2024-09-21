use anyhow::Result;
use clap::Parser;
use godam::{commands::*, Cli};

#[tokio::main]
async fn main() -> Result<()> {
    let cli = Cli::parse();

    match &cli.command {
        Commands::Init => init::run()?,
        Commands::Search { name } => search::run(name).await?,
        Commands::Install { name } => install::run(name).await?,
        Commands::Uninstall { name } => uninstall::run(name).await?,
        Commands::List => list::run()?,
        Commands::Clean => clean::run()?,
    }

    Ok(())
}
