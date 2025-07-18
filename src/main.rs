mod commands;
mod env;
mod google_sheet_api_client;
mod models;
mod utils;

extern crate google_sheets4 as sheets4;

use crate::utils::constants::{APPNAME, DESCRIPTION};
use clap::{Parser, Subcommand};

#[derive(Parser)]
#[command(name = APPNAME, version, about = DESCRIPTION)]
#[command(arg_required_else_help = true)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    #[command(name = "setup", about = "Setup the fetcher")]
    Setup,
    #[command(name = "sync", about = "Sync latest available translations")]
    Sync,
}

#[tokio::main]
async fn main() {
    dotenvy::dotenv().ok();

    let cli = Cli::parse();

    match cli.command {
        Commands::Setup => {
            commands::setup::Setup::run().await;
        }
        Commands::Sync => {
            commands::sync::Sync::run().await;
        }
    }
}
