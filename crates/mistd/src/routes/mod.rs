use std::sync::Arc;

use axum::Router;

use crate::state::AppState;

pub mod droplet;

pub fn router() -> Router<Arc<AppState>> {
    Router::new().nest("/droplet", droplet::router())
}
