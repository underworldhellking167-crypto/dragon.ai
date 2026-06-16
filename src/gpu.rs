 use anyhow::Result;

#[derive(Debug, Clone)]
pub struct GpuConfig {
    pub enabled: bool,
    pub backend: String,
}

impl GpuConfig {
    pub fn report(&self) -> Result<()> {
        println!("GPU enabled: {}", self.enabled);
        println!("GPU backend: {}", self.backend);
        println!("(GPU backend manager placeholder)");
        Ok(())
    }
}