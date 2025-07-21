pub mod create;
pub mod execute;

use std::sync::Arc;

use axum::{
    Router,
    routing::{get, post},
};

use crate::state::AppState;

pub fn router() -> Router<Arc<AppState>> {
    Router::new()
        .route("/", post(create::handler))
        .route("/{id}/execute", get(execute::handler))
}
