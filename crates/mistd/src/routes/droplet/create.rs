use std::sync::Arc;

use axum::{Json, extract::State, http::StatusCode, response::IntoResponse};
use config::RootConfig;
use serde::Deserialize;

use crate::state::AppState;

#[derive(Debug, Deserialize)]
pub struct DropletCreatePayload {
    config: RootConfig,
}

pub async fn handler(
    State(state): State<Arc<AppState>>,
    Json(payload): Json<DropletCreatePayload>,
) -> impl IntoResponse {
    if payload.config.spec.as_droplet().is_none() {
        panic!("Not a droplet config.");
    }

    let name = payload.config.metadata.name.clone();

    state
        .control_panel()
        .create_droplet(payload.config)
        .unwrap();

    tracing::info!("Created droplet: {}", name);

    StatusCode::OK
}
