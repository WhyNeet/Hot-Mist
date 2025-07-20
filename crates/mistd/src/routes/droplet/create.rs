use std::{fs, sync::Arc};

use axum::{Json, extract::State, http::StatusCode, response::IntoResponse};
use config::{RootConfig, Spec, SpecSource};
use serde::Deserialize;
use wasmtime::component::Component;

use crate::state::AppState;

#[derive(Debug, Deserialize)]
pub struct DropletCreatePayload {
    config: RootConfig,
}

pub async fn handler(
    State(state): State<Arc<AppState>>,
    Json(payload): Json<DropletCreatePayload>,
) -> impl IntoResponse {
    let Spec::Droplet { source, .. } = payload.config.spec;

    let artifact = match source {
        SpecSource::File { path } => {
            let bytes = fs::read(path).unwrap();
            state.engine().precompile_component(&bytes).unwrap()
        }
    };
    let component = unsafe { Component::deserialize(state.engine(), artifact) }.unwrap();

    state
        .artifacts()
        .insert(payload.config.metadata.name.clone(), component);

    tracing::info!("Created droplet: {}", payload.config.metadata.name);

    StatusCode::OK
}
