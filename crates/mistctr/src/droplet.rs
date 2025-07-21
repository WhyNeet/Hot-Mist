use std::{fs, sync::Arc};

use config::{RootConfig, Spec, SpecSource};
use wasmtime::{
    Engine, Store,
    component::{Component, Linker},
};
use wasmtime_wasi::{
    ResourceTable,
    p2::{WasiCtxBuilder, pipe::MemoryOutputPipe},
};

use crate::{context::ControlContext, state::HostState};

pub struct DropletHandle {
    pub config: RootConfig,
    component: Component,
    linker: Arc<Linker<HostState>>,
    engine: Engine,
}

impl DropletHandle {
    pub fn new(cx: &ControlContext, config: RootConfig) -> anyhow::Result<Self> {
        let Spec::Droplet { source, .. } = &config.spec;

        let bytes = match source {
            SpecSource::File { path } => fs::read(path)?,
        };

        let artifact = cx.engine().precompile_component(&bytes)?;
        fs::write(cx.artifact_dir().join(&config.metadata.name), &artifact)?;

        let component = unsafe { Component::deserialize(cx.engine(), artifact) }?;

        let mut linker = Linker::new(cx.engine());
        wasmtime_wasi::p2::add_to_linker_async(&mut linker)?;

        Ok(Self {
            config,
            engine: cx.engine().clone(),
            component,
            linker: Arc::new(linker),
        })
    }

    pub async fn run(&self) -> anyhow::Result<DropletExecutionResult> {
        let stdout = MemoryOutputPipe::new(4096);

        let table = ResourceTable::new();
        let mut ctx = WasiCtxBuilder::new();
        if let Some((_, runtime, _)) = self.config.spec.as_droplet() {
            if let Some(ref env) = runtime.env {
                for var in env {
                    ctx.env(var.name.clone(), var.value.clone());
                }
            }
        }

        let ctx = ctx.stdout(stdout.clone()).build();

        let state = HostState { ctx, table };
        let mut store = Store::new(&self.engine, state);

        let instance = self
            .linker
            .instantiate_async(&mut store, &self.component)
            .await?;
        let handler = instance.get_func(&mut store, "handler").unwrap();
        handler.call_async(store, &[], &mut []).await?;

        let stdout = String::from_utf8(stdout.contents().to_vec())?;

        Ok(DropletExecutionResult { stdout })
    }
}

#[derive(Debug)]
pub struct DropletExecutionResult {
    pub stdout: String,
}
