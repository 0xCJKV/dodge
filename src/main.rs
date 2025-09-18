use anyhow::Result;
use clap::Parser;

mod server;
mod config;
mod cli;
mod theme;
mod generator;
mod content;
mod utils;

use cli::{Cli, CommandHandler};

#[tokio::main]
async fn main() -> Result<()> {
    let cli = Cli::parse();
    CommandHandler::handle_command(cli.command).await
}