use clap::Parser;
use console::style;
use godam::{Cli, ORANGE};

#[tokio::main]
async fn main() {
    let cli = Cli::parse();

    godam::run(&cli.command).await.unwrap_or_else(|e| {
        let msg = style(format!("godam: {e}")).color256(ORANGE);
        println!("{msg}");
    })
}
