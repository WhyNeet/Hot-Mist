use std::sync::Arc;

use axum::{Json, extract::State, http::StatusCode, response::IntoResponse};
use serde::Deserialize;
use wasmtime::component::Component;

use crate::state::AppState;

#[derive(Debug, Deserialize)]
pub struct DropletDefinePayload {
    name: String,
    wasm: Vec<u8>,
}

pub async fn handler(
    State(state): State<Arc<AppState>>,
    Json(payload): Json<DropletDefinePayload>,
) -> impl IntoResponse {
    let artifact = state.engine().precompile_component(&payload.wasm).unwrap();
    let component = unsafe { Component::deserialize(state.engine(), artifact) }.unwrap();

    state.artifacts().insert(payload.name.clone(), component);

    tracing::info!("Created droplet: {}", payload.name);

    StatusCode::OK
}
