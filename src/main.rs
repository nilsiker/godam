use anyhow::Result;
use clap::Parser;
use gaddon::{commands::*, Cli};

#[tokio::main]
async fn main() -> Result<()>  {
    let cli = Cli::parse();

    match &cli.command {
        Commands::Add { name } => {
            add(name).await?
        },
        Commands::Init => init(),
        Commands::Install => install(),
        Commands::Rm {name} => rm(name),
    }

    Ok(())
}
