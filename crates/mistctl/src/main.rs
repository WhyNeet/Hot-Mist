use clap::Parser;
use mistctl::{
    args::{Args, Command},
    commands::droplet,
};
use serde::{Deserialize, Serialize};

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let args = Args::parse();
    match args.command {
        Command::Droplet { command } => droplet::droplet_cmd(command).await,
    }
}

#[derive(Debug, Deserialize, Serialize)]
pub struct DropletDefinePayload {
    name: String,
    wasm: Vec<u8>,
}
