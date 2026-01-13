pub mod args;
pub mod asset;
mod commands;
mod config;
mod console;
mod fs;
mod godot;
pub mod job;
mod traits;
mod web_requests;

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
    pub command: Command,
}

pub async fn run(command: &Command) -> Result<(), Box<dyn std::error::Error>> {
    match command {
        Command::Init => init::exec()?,
        Command::Search { name } => search::exec(name).await?,
        Command::Add {
            id,
            source,
            cache,
            force,
            include,
            exclude,
        } => todo!(),
        Command::Install {
            id,
            source,
            cache,
            force,
            include,
            exclude,
        } => install::exec(id, source, cache, *force, include.clone(), exclude.clone()).await?,
        Command::Uninstall { id } => uninstall::exec(id)?,
        Command::List => list::exec()?,
        Command::Clean => clean::exec()?,
    };

    Ok(())
}
