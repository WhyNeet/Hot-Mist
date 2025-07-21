use std::{fs, sync::Arc};

use config::{RootConfig, Spec, SpecSource};
use tokio::io::AsyncReadExt;
use wasmtime::{
    Engine, Store,
    component::{Component, Linker},
};
use wasmtime_wasi::{
    ResourceTable,
    p2::{AsyncStdoutStream, WasiCtxBuilder, pipe::AsyncWriteStream},
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

        let artifact_path = cx.storage().artifact_dir.join(&config.metadata.name);
        let artifact = if fs::exists(&artifact_path)? {
            fs::read(artifact_path)?
        } else {
            let bytes = match source {
                SpecSource::File { path } => fs::read(path)?,
            };

            let artifact = cx.engine().precompile_component(&bytes)?;
            fs::write(
                cx.storage().artifact_dir.join(&config.metadata.name),
                &artifact,
            )?;
            artifact
        };

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
        let (mut reader, writer) = tokio::io::duplex(65536);
        let stdout = AsyncStdoutStream::new(AsyncWriteStream::new(16384, writer));

        let table = ResourceTable::new();
        let mut ctx = WasiCtxBuilder::new();
        if let Some((_, runtime, _)) = self.config.spec.as_droplet() {
            if let Some(ref env) = runtime.env {
                for var in env {
                    ctx.env(var.name.clone(), var.value.clone());
                }
            }
        }

        let ctx = ctx.stdout(stdout).build();

        let state = HostState { ctx, table };
        let mut store = Store::new(&self.engine, state);

        let instance = self
            .linker
            .instantiate_async(&mut store, &self.component)
            .await?;
        let handler = instance.get_func(&mut store, "handler").unwrap();
        handler.call_async(store, &[], &mut []).await?;

        let mut stdout = vec![];
        reader.read_to_end(&mut stdout).await?;
        let stdout = String::from_utf8(stdout)?;

        Ok(DropletExecutionResult { stdout })
    }
}

#[derive(Debug)]
pub struct DropletExecutionResult {
    pub stdout: String,
}
