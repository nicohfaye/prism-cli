use anyhow::{Context, Result};
use serde::Deserialize;
use std::path::PathBuf;

#[derive(Debug, Deserialize)]
pub struct Config {
    pub ssh: SshConfig,
    pub kubernetes: KubernetesConfig,
}

#[derive(Debug, Deserialize)]
pub struct SshConfig {
    pub host: String,
    pub user: String,
    #[serde(default = "default_ssh_port")]
    pub port: u16,
    pub key_path: Option<String>,
}

#[derive(Debug, Deserialize)]
pub struct KubernetesConfig {
    pub kubeconfig: String,
    #[serde(default = "default_api_port")]
    pub api_port: u16,
    #[serde(default = "default_local_port")]
    pub local_port: u16,
}

fn default_ssh_port() -> u16 {
    22
}

fn default_api_port() -> u16 {
    6443
}

fn default_local_port() -> u16 {
    16443
}

impl Config {
    pub fn load() -> Result<Self> {
        let config_path = Self::config_path();
        let content = std::fs::read_to_string(&config_path)
            .with_context(|| format!("Failed to read config at {}", config_path.display()))?;
        let config: Config =
            toml::from_str(&content).with_context(|| "Failed to parse config.toml")?;
        Ok(config)
    }

    fn config_path() -> PathBuf {
        let mut dir = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
        dir.push("config.toml");
        dir
    }

    /// Resolve the kubeconfig path relative to the project directory,
    /// or as an absolute / `~`-expanded path.
    pub fn kubeconfig_path(&self) -> PathBuf {
        let raw = &self.kubernetes.kubeconfig;
        if let Some(stripped) = raw.strip_prefix("~/") {
            if let Some(home) = dirs::home_dir() {
                return home.join(stripped);
            }
        }
        let path = PathBuf::from(raw);
        if path.is_absolute() {
            path
        } else {
            PathBuf::from(env!("CARGO_MANIFEST_DIR")).join(path)
        }
    }
}
