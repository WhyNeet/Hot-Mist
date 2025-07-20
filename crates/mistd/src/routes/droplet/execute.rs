use std::sync::Arc;

use anyhow::Context;
use axum::{
    extract::{Path, State},
    http::StatusCode,
    response::IntoResponse,
};
use wasmtime::{Store, component::Linker};
use wasmtime_wasi::{
    ResourceTable,
    p2::{IoView, WasiCtx, WasiView, pipe::MemoryOutputPipe},
};

use crate::state::AppState;

pub struct WasiState {
    ctx: WasiCtx,
    table: ResourceTable,
}

impl IoView for WasiState {
    fn table(&mut self) -> &mut ResourceTable {
        &mut self.table
    }
}
impl WasiView for WasiState {
    fn ctx(&mut self) -> &mut WasiCtx {
        &mut self.ctx
    }
}

pub async fn handler(
    State(state): State<Arc<AppState>>,
    Path(id): Path<String>,
) -> impl IntoResponse {
    let component = state.artifacts().get(&id).unwrap();

    let mut linker = Linker::<WasiState>::new(state.engine());
    wasmtime_wasi::p2::add_to_linker_async(&mut linker).unwrap();

    let stdout_pipe = MemoryOutputPipe::new(4096);

    let wasi = wasmtime_wasi::p2::WasiCtxBuilder::new()
        .stdout(stdout_pipe.clone())
        .build();
    let wasi_state = WasiState {
        ctx: wasi,
        table: ResourceTable::new(),
    };
    let mut store = Store::new(state.engine(), wasi_state);

    let instance = linker
        .instantiate_async(&mut store, &component)
        .await
        .unwrap();

    let func = instance
        .get_func(&mut store, "handler")
        .context("Unable to locate `handler` function.")
        .unwrap();
    let mut result = [];
    func.call_async(&mut store, &[], &mut result).await.unwrap();

    let stdout_bytes = stdout_pipe.contents();
    let stdout_string = String::from_utf8(stdout_bytes.to_vec()).unwrap();

    (StatusCode::OK, stdout_string)
}
