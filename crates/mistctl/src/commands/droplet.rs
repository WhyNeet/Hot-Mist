use std::{env, fs};

use config::{RootConfig, Spec, SpecSource};
use reqwest::{Client, StatusCode};
use serde_json::json;

use crate::args::DropletCommand;

pub async fn droplet_cmd(command: DropletCommand) -> anyhow::Result<()> {
    match command {
        DropletCommand::Create { config } => {
            let config = fs::read_to_string(config)?;
            let mut config: RootConfig = serde_yaml::from_str(&config)?;

            match &mut config.spec {
                Spec::Droplet { source, .. } => match source {
                    SpecSource::File { path } => {
                        if path.is_relative() {
                            *path = env::current_dir()?.join(path.clone());
                        }
                    }
                },
            }

            match create_droplet(&config).await? {
                code if code.is_success() => println!("Created."),
                other => eprintln!("Failed to create droplet ({other})."),
            }
        }
        DropletCommand::Execute { name } => todo!("exec: {name}"),
    }

    Ok(())
}

pub async fn create_droplet(config: &RootConfig) -> anyhow::Result<StatusCode> {
    let client = Client::new();
    let request = client
        .post("http://0.0.0.0:8080/ctr/droplet")
        .json(&json!({ "config": config }));
    let response = request.send().await?;

    Ok(response.status())
}
