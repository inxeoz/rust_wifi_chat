mod server;
mod client;

use clap::{Parser, Subcommand};
use std::process::Command;

/// Wi-Fi Manager CLI
#[derive(Parser)]
#[command(name = "wifi-cli")]
#[command(about = "A simple CLI tool to manage Wi-Fi using terminal commands", long_about = None)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {

    Connect {
        ip: String,
        port: u16,
    },

    Host {
        ip_category: String,
        port: u16,
    }
}

fn main() {
    let cli = Cli::parse();

    match &cli.command {
        Commands::Connect { ip, port } => {
            client::connect_server(ip, port);
        },
        Commands::Host { port , ip_category} => {
            server::host_server(ip_category, port);
        }
    }
}

