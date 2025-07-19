use std::env;

use anyhow::Context;
use wasmtime::{
    Cache, CacheConfig, Config, Engine, Store,
    component::{Component, Linker},
};
use wasmtime_wasi::{
    ResourceTable,
    p2::{IoView, WasiCtx, WasiView},
};

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let mut config = Config::new();
    config.async_support(true);
    let mut cache = CacheConfig::new();
    cache.with_directory(env::current_dir()?.join("aot_cache"));
    config.cache(Some(Cache::new(cache)?));

    // 1. Wasmtime Engine + module
    let engine = Engine::new(&config)?;

    let component = if std::fs::exists("test.cwasm")? {
        let bytes = std::fs::read("test.cwasm")?;
        println!("Using precompiled artifact.\n");
        unsafe { Component::deserialize(&engine, &bytes) }?
    } else {
        let component = std::fs::read("test.wasm")?;
        let bytes = engine.precompile_component(&component)?;
        std::fs::write("./test.cwasm", &bytes)?;
        unsafe { Component::deserialize(&engine, &bytes) }?
    };

    // 2. WASI configuration
    let mut linker = Linker::<WasiState>::new(&engine);
    wasmtime_wasi::p2::add_to_linker_async(&mut linker)?;

    let wasi = wasmtime_wasi::p2::WasiCtxBuilder::new()
        .inherit_stdout()
        .build();
    let state = WasiState {
        ctx: wasi,
        table: ResourceTable::new(),
    };
    let mut store = Store::new(&engine, state);

    let instance = linker.instantiate_async(&mut store, &component).await?;

    let func = instance
        .get_func(&mut store, "handler")
        .context("Unable to locate `handler` function.")?;
    let mut result = [wasmtime::component::Val::String(String::new())];
    func.call_async(&mut store, &[], &mut result).await?;

    println!("Output: {:?}", result[0]);

    Ok(())
}

struct WasiState {
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
