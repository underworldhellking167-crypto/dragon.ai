 use anyhow::{Context, Result};
 use colored::*;
 use dirs::home_dir;
 use serde::{Deserialize, Serialize};
 use std::{fs, path::PathBuf};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Config {
    pub architecture: String,
    pub params: String,
    pub context_len: usize,
    pub gpu: bool,
    pub gpu_backend: String,
    pub cpu_threads: usize,
    pub thinking: bool,
    pub reasoning: bool,
    pub api_server: bool,
    pub api_port: u16,
    pub memory_mode: String,
}

impl Default for Config {
    fn default() -> Self {
        Config {
            architecture: "default".to_string(),
            params: "./params/.gitkeep".to_string(),
            context_len: 8192,
            gpu: false,
            gpu_backend: "cuda".to_string(),
            cpu_threads: 8,
            thinking: true,
            reasoning: true,
            api_server: false,
            api_port: 8080,
            memory_mode: "stream".to_string(),
        }
    }
}

impl Config {
    pub fn config_path() -> Result<PathBuf> {
        let mut config_path = home_dir().context("Could not determine home directory")?;
        config_path.push(".dragon");
        fs::create_dir_all(&config_path).context("Failed to create config directory")?;
        config_path.push("config.yml");
        Ok(config_path)
    }

    pub fn load() -> Result<Self> {
        let path = Self::config_path()?;
        if !path.exists() {
            let default = Config::default();
            default.save()?;
            return Ok(default);
        }

        let contents = fs::read_to_string(&path)
            .with_context(|| format!("Failed to read config file {}", path.display()))?;
        let config: Config = serde_yaml::from_str(&contents)
            .with_context(|| "Failed to parse config.yml")?;
        Ok(config)
    }

    pub fn save(&self) -> Result<()> {
        let path = Self::config_path()?;
        let contents = serde_yaml::to_string(self).context("Failed to serialize config")?;
        fs::write(&path, contents).with_context(|| format!("Failed to write config file {}", path.display()))?;
        Ok(())
    }

    pub fn print(&self) -> Result<()> {
        println!("{}", "Dragon.AI Configuration".bold().cyan());
        println!("{} {}", "Architecture:".green(), self.architecture);
        println!("{} {}", "Params:".green(), self.params);
        println!("{} {}", "Context Len:".green(), self.context_len);
        println!("{} {}", "GPU:".green(), self.gpu);
        println!("{} {}", "GPU Backend:".green(), self.gpu_backend);
        println!("{} {}", "CPU Threads:".green(), self.cpu_threads);
        println!("{} {}", "Thinking:".green(), self.thinking);
        println!("{} {}", "Reasoning:".green(), self.reasoning);
        println!("{} {}", "API Server:".green(), self.api_server);
        println!("{} {}", "API Port:".green(), self.api_port);
        println!("{} {}", "Memory Mode:".green(), self.memory_mode);
        Ok(())
    }
}