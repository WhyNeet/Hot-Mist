use std::{fs, path::PathBuf};

use wasmtime::{Config, Engine};

pub struct ControlContext {
    artifact_dir: PathBuf,
    engine: Engine,
}

impl ControlContext {
    pub fn new(artifact_dir: PathBuf, config: &Config) -> anyhow::Result<Self> {
        if !fs::exists(&artifact_dir)? {
            fs::create_dir_all(&artifact_dir)?;
        }

        let engine = Engine::new(config)?;

        Ok(Self {
            artifact_dir,
            engine,
        })
    }

    pub fn artifact_dir(&self) -> &PathBuf {
        &self.artifact_dir
    }

    pub fn engine(&self) -> &Engine {
        &self.engine
    }
}
