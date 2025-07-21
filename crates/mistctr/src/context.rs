use std::{fs, path::PathBuf};

use wasmtime::{Config, Engine};

pub struct ControlContext {
    storage: StorageContext,
    engine: Engine,
}

impl ControlContext {
    pub fn new(root_dir: PathBuf, config: &Config) -> anyhow::Result<Self> {
        let storage = StorageContext::create(root_dir)?;

        let engine = Engine::new(config)?;

        Ok(Self { storage, engine })
    }

    pub fn engine(&self) -> &Engine {
        &self.engine
    }

    pub fn storage(&self) -> &StorageContext {
        &self.storage
    }
}

pub struct StorageContext {
    pub root_dir: PathBuf,
    pub artifact_dir: PathBuf,
    pub config_dir: PathBuf,
}

impl StorageContext {
    pub fn create(root_dir: PathBuf) -> anyhow::Result<Self> {
        let storage = Self {
            artifact_dir: root_dir.join("artifacts"),
            config_dir: root_dir.join("config"),
            root_dir,
        };

        if !fs::exists(&storage.root_dir)? {
            fs::create_dir_all(&storage.root_dir)?;
        }

        if !fs::exists(&storage.artifact_dir)? {
            fs::create_dir_all(&storage.artifact_dir)?;
        }

        Ok(storage)
    }
}
