
mod peer;

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
        ip: String,
        port: u16,
    }
}

fn main() {
    let cli = Cli::parse();

    match &cli.command {
        Commands::Connect { ip, port } => {

            let address = format!("{}:{}", ip, port);
            peer::connect_to_peer(&address);
        },
        Commands::Host { port , ip} => {
            let address = format!("{}:{}", ip, port);
           peer::start_listener(&address);
        }
    }
}

