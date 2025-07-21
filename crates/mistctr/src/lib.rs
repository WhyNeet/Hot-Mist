use std::env;

use anyhow::Context;
use config::RootConfig;
use dashmap::DashMap;
use wasmtime::Config;

use crate::{
    context::ControlContext,
    droplet::{DropletExecutionResult, DropletHandle},
};

pub mod context;
pub mod droplet;
pub mod state;

pub struct ControlPanel {
    droplets: DashMap<String, DropletHandle>,
    cx: ControlContext,
}

impl ControlPanel {
    pub fn default() -> anyhow::Result<Self> {
        let mut config = Config::new();
        config.async_support(true);
        let cx = ControlContext::new(env::current_dir()?.join("artifacts"), &config)?;

        Ok(Self {
            droplets: DashMap::new(),
            cx,
        })
    }
}

impl ControlPanel {
    pub async fn run_droplet(&self, name: &str) -> anyhow::Result<DropletExecutionResult> {
        let droplet = self.droplets.get(name).context("Droplet does not exist.")?;

        droplet.run().await
    }

    pub fn create_droplet(&self, config: RootConfig) -> anyhow::Result<()> {
        let name = config.metadata.name.clone();
        let handle = DropletHandle::new(&self.cx, config)?;

        self.droplets.insert(name, handle);

        Ok(())
    }
}
