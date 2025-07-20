use std::{env, fs, path::PathBuf};

use reqwest::{Client, StatusCode};
use serde::{Deserialize, Serialize};

#[tokio::main]
async fn main() {
    let mut args = env::args().skip(1);

    let name = args.next().expect("Expected artifact name.");
    let path: PathBuf = args
        .next()
        .expect("Expected WASM file path.")
        .parse()
        .unwrap();

    let wasm = fs::read(path).unwrap();

    let client = Client::new();
    let request = client
        .post("http://0.0.0.0:8080/ctr/droplet")
        .json(&DropletDefinePayload { name, wasm });
    let response = request.send().await.unwrap();

    match response.status() {
        StatusCode::OK => println!("success"),
        other => println!("failure: {other}"),
    }
}

#[derive(Debug, Deserialize, Serialize)]
pub struct DropletDefinePayload {
    name: String,
    wasm: Vec<u8>,
}
