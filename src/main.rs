use clap::Parser;
use godam::Cli;

#[tokio::main]
async fn main() {
    let cli = Cli::parse();
    
    godam::run(&cli.command)
        .await
        .unwrap_or_else(|e| println!("{e}"));
}
