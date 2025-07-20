use std::sync::Arc;

use anyhow::anyhow;
use axum::Router;
use mistd::{routes, state::AppState};
use tokio::net::TcpListener;
use tracing::log::LevelFilter;
use wasmtime::Config;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    env_logger::Builder::default()
        .filter_level(LevelFilter::Info)
        .parse_env("APP_LOG")
        .init();

    let mut config = Config::new();
    config.async_support(true);
    let state = AppState::new(&config)?;
    let state = Arc::new(state);

    let router = Router::new()
        .nest("/ctr", routes::router())
        .with_state(state);

    let listener = TcpListener::bind(("0.0.0.0", 8080)).await?;

    tracing::info!("Listening on 0.0.0.0:8080");

    axum::serve(listener, router).await.map_err(|e| anyhow!(e))

    // let mut config = Config::new();
    // config.async_support(true);

    // // 1. Wasmtime Engine + Component
    // let engine = Engine::new(&config)?;

    // let component = if std::fs::exists("test.cwasm")? {
    //     let bytes = std::fs::read("test.cwasm")?;
    //     println!("Using precompiled artifact.\n");
    //     unsafe { Component::deserialize(&engine, &bytes) }?
    // } else {
    //     let component = std::fs::read("test.wasm")?;
    //     let bytes = engine.precompile_component(&component)?;
    //     std::fs::write("./test.cwasm", &bytes)?;
    //     unsafe { Component::deserialize(&engine, &bytes) }?
    // };

    // // 2. WASI configuration
    // let mut linker = Linker::<WasiState>::new(&engine);
    // wasmtime_wasi::p2::add_to_linker_async(&mut linker)?;

    // let wasi = wasmtime_wasi::p2::WasiCtxBuilder::new()
    //     .inherit_stdout()
    //     .build();
    // let state = WasiState {
    //     ctx: wasi,
    //     table: ResourceTable::new(),
    // };
    // let mut store = Store::new(&engine, state);

    // let instance = linker.instantiate_async(&mut store, &component).await?;

    // let func = instance
    //     .get_func(&mut store, "handler")
    //     .context("Unable to locate `handler` function.")?;
    // let mut result = [wasmtime::component::Val::String(String::new())];
    // func.call_async(&mut store, &[], &mut result).await?;

    // println!("Output: {:?}", result[0]);

    // Ok(())
}
