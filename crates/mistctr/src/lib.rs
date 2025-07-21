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
    db: sled::Db,
}

impl ControlPanel {
    pub fn default() -> anyhow::Result<Self> {
        let mut config = Config::new();
        config.async_support(true);
        let cx = ControlContext::new(env::current_dir()?.join("mist"), &config)?;

        let db = sled::open(cx.storage().root_dir.join("db"))?;
        let droplets = Self::load_droplets_state(db.clone())?;
        let droplets =
            droplets
                .into_iter()
                .try_fold(DashMap::new(), |acc, val| -> anyhow::Result<_> {
                    acc.insert(val.metadata.name.clone(), DropletHandle::new(&cx, val)?);
                    anyhow::Result::Ok(acc)
                })?;

        Ok(Self { droplets, db, cx })
    }

    fn load_droplets_state(db: sled::Db) -> anyhow::Result<Vec<RootConfig>> {
        db.iter()
            .map(|item| {
                let (_, value) = item?;
                let config = serde_json::from_slice(&value)?;

                Ok(config)
            })
            .collect::<anyhow::Result<Vec<_>>>()
    }
}

impl ControlPanel {
    pub async fn run_droplet(&self, name: &str) -> anyhow::Result<DropletExecutionResult> {
        let droplet = self.droplets.get(name).context("Droplet does not exist.")?;

        droplet.run().await
    }

    pub fn create_droplet(&self, config: RootConfig) -> anyhow::Result<()> {
        let name = config.metadata.name.clone();

        self.db.insert(name.clone(), serde_json::to_vec(&config)?)?;

        let handle = DropletHandle::new(&self.cx, config)?;
        self.droplets.insert(name, handle);

        Ok(())
    }
}

impl Drop for ControlPanel {
    fn drop(&mut self) {
        self.db.flush().expect("failed to flush db");
    }
}
