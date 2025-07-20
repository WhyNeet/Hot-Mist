use std::path::PathBuf;

use clap::{Parser, Subcommand};

#[derive(Debug, Parser)]
pub struct Args {
    #[command(subcommand)]
    pub command: Command,
}

#[derive(Debug, Subcommand)]
pub enum Command {
    Droplet {
        #[command(subcommand)]
        command: DropletCommand,
    },
}

#[derive(Debug, Subcommand)]
pub enum DropletCommand {
    Create {
        #[arg(index = 1)]
        config: PathBuf,
    },
    Execute {
        #[arg(index = 1)]
        name: String,
    },
}
