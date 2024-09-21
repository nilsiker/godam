use anyhow::Result;
use clap::Parser;
use godam::{commands::*, Cli};

#[tokio::main]
async fn main() -> Result<()> {
    let cli = Cli::parse();

    match &cli.command {
        Commands::Init => init::run()?,
        Commands::Install {name} => install::run(name).await?,
        Commands::Uninstall => todo!(),
        Commands::Clean => todo!(),
    }

    Ok(())
}
