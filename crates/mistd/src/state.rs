use dashmap::DashMap;
use wasmtime::{Config, Engine, component::Component};

#[derive(Clone)]
pub struct AppState {
    engine: Engine,
    artifacts: DashMap<String, Component>,
}

impl AppState {
    pub fn new(config: &Config) -> anyhow::Result<Self> {
        Ok(Self {
            engine: Engine::new(config)?,
            artifacts: DashMap::new(),
        })
    }

    pub fn engine(&self) -> &Engine {
        &self.engine
    }

    pub fn artifacts(&self) -> &DashMap<String, Component> {
        &self.artifacts
    }
}
