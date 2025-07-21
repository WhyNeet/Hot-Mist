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
        DropletCommand::Execute { name } => {
            println!("Executing Droplet: {name}");
            let stdout = execute_droplet(&name).await?;
            println!("=== stdout ===");
            println!("{stdout}");
        }
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

pub async fn execute_droplet(name: &str) -> anyhow::Result<String> {
    let client = Client::new();
    let request = client.get(format!("http://0.0.0.0:8080/ctr/droplet/{name}/execute"));
    let response = request.send().await?;
    let status = response.status();

    let contents = response.text().await?;

    if !status.is_success() {
        anyhow::bail!("Failed to execute droplet ({}):\n{}", status, contents);
    }

    Ok(contents)
}
