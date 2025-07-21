use std::sync::Arc;

use anyhow::anyhow;
use axum::Router;
use mistctr::ControlPanel;
use mistd::{routes, state::AppState};
use tokio::net::TcpListener;
use tracing::log::LevelFilter;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    env_logger::Builder::default()
        .filter_level(LevelFilter::Info)
        .parse_env("APP_LOG")
        .init();

    let control_panel = Arc::new(ControlPanel::default()?);

    tokio::spawn(async move {
        let state = AppState::new(control_panel);
        let state = Arc::new(state);

        let router = Router::new()
            .nest("/ctr", routes::router())
            .with_state(state);

        let listener = TcpListener::bind(("0.0.0.0", 8080)).await?;

        tracing::info!("Listening on 0.0.0.0:8080");

        axum::serve(listener, router).await.map_err(|e| anyhow!(e))
    })
    .await?
}
