mod api;
mod assets;
mod cache;
mod commands;
mod config;
mod godot;
mod traits;

use clap::Parser;
use commands::*;

#[derive(Parser)]
#[command(version, about, long_about = None, arg_required_else_help = true)]
#[command(propagate_version = true)]
/// godam
///
/// A minimal addon manager for Godot.
pub struct Cli {
    #[command(subcommand)]
    pub command: Commands,
}

pub async fn run(command: &Commands) -> Result<(), Box<dyn std::error::Error>> {
    match command {
        Commands::Init => init::run()?,
        Commands::Search { name } => search::run(name).await?,
        Commands::Install { name } => install::run(name).await?,
        Commands::Uninstall { name } => uninstall::run(name).await?,
        Commands::List => list::run()?,
        Commands::Clean => clean::run()?,
    }

    Ok(())
}
