use anyhow::Result;
use clap::Parser;
use godam::{commands::*, Cli};

#[tokio::main]
async fn main() -> Result<()> {
    let cli = Cli::parse();

    match &cli.command {
        Commands::Add { name } => match add(name).await {
            Ok(()) => println!("Successfully added addon."),
            Err(e) => println!("{e}"),
        },
        Commands::Init => init()?,
        Commands::Install => install().await?,
        Commands::Rm { name } => rm(name)?,
    }

    Ok(())
}
