pub mod define;
pub mod execute;

use std::sync::Arc;

use axum::{Router, routing::post};

use crate::state::AppState;

pub fn router() -> Router<Arc<AppState>> {
    Router::new()
        .route("/", post(define::handler))
        .route("/{id}/execute", post(execute::handler))
}
