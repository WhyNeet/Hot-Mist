use std::sync::Arc;

use crate::state::AppState;
use axum::{
    extract::{Path, State},
    http::StatusCode,
    response::IntoResponse,
};

pub async fn handler(
    State(state): State<Arc<AppState>>,
    Path(id): Path<String>,
) -> impl IntoResponse {
    let output = match state.control_panel().run_droplet(&id).await {
        Ok(result) => result,
        Err(e) => {
            tracing::error!("{e}");
            return (StatusCode::INTERNAL_SERVER_ERROR, e.to_string());
        }
    };

    (StatusCode::OK, output.stdout)
}
