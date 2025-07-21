use serde::{Deserialize, Serialize};
use std::path::PathBuf;

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct RootConfig {
    pub api_version: String,
    pub metadata: Metadata,
    #[serde(flatten)]
    pub spec: Spec,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(tag = "kind", content = "spec")]
pub enum Spec {
    Droplet {
        source: SpecSource,
        runtime: SpecRuntime,
        secrets: Vec<SpecSecret>,
    },
}

impl Spec {
    pub fn as_droplet(&self) -> Option<(&SpecSource, &SpecRuntime, &Vec<SpecSecret>)> {
        match self {
            Self::Droplet {
                source,
                runtime,
                secrets,
            } => Some((source, runtime, secrets)),
        }
    }
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct SpecSecret {
    pub name: String,
    pub mount_path: PathBuf,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(untagged)]
pub enum SpecSource {
    #[serde(rename = "File")]
    File { path: PathBuf },
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Metadata {
    pub name: String,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct SpecRuntime {
    pub env: Option<Vec<RuntimeEnv>>,
    pub filesystem: Option<Vec<RuntimeFilesystemMount>>,
    pub network: Option<RuntimeNetwork>,
    pub resources: RuntimeResources,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct RuntimeEnv {
    pub name: String,
    pub value: String,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct RuntimeFilesystemMount {
    pub name: String,
    pub guest_path: String,
    pub host_path: String,
    pub read_only: bool,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct RuntimeNetwork {
    pub allowed_hosts: Vec<String>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct RuntimeResources {
    pub memory: String,
    pub cpu: String,
}
