pub mod api;
pub mod assets;
pub mod commands;
pub mod config;

use clap::Parser;
use commands::Commands;

#[derive(Parser)]
#[command(version, about, long_about = None, arg_required_else_help = true)]
#[command(propagate_version = true)]
/// gaddon
///
/// A minimal addon manager for Godot.
pub struct Cli {
    #[command(subcommand)]
    pub command: Commands,
}