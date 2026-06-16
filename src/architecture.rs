 use anyhow::{Context, Result};
 use serde::{Deserialize, Serialize};
 use std::{fs, path::{Path, PathBuf}};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Architecture {
    pub name: String,
    pub architecture: ArchitectureSpec,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct ArchitectureSpec {
    pub r#type: String,
    pub layers: usize,
    pub hidden_size: usize,
    pub attention_heads: usize,
    pub kv_heads: usize,
    pub vocab_size: usize,
    pub max_position: usize,
    pub norm: String,
    pub activation: String,
}

impl Architecture {
    pub fn load(path: impl AsRef<Path>) -> Result<Self> {
        let data = fs::read_to_string(&path)
            .with_context(|| format!("Failed to read architecture file {}", path.as_ref().display()))?;
        let architecture: Architecture = serde_json::from_str(&data)
            .with_context(|| "Failed to parse architecture JSON")?;
        Ok(architecture)
    }

    pub fn default_path() -> PathBuf {
        PathBuf::from("architectures/default.json")
    }

    pub fn summary(&self) -> String {
        format!(
            "{}: {} layers, {} hidden, {} heads, {} max position, {} vocab",
            self.name,
            self.architecture.layers,
            self.architecture.hidden_size,
            self.architecture.attention_heads,
            self.architecture.max_position,
            self.architecture.vocab_size,
        )
    }
}