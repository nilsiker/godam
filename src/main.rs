use anyhow::Result;
use clap::Parser;
use gaddon::{commands::*, Cli};

#[tokio::main]
async fn main() -> Result<()> {
    let cli = Cli::parse();

    match &cli.command {
        Commands::Add { name } => match add(name).await {
            Ok(()) => println!("Successfully added addon."),
            Err(e) => println!("{e}"),
        },
        Commands::Init => init(),
        Commands::Install => install(),
        Commands::Rm { name } => rm(name),
    }

    Ok(())
}
