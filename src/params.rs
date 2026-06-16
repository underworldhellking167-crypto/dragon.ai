 use anyhow::{Context, Result};
 use memmap2::{Mmap, MmapOptions};
 use std::{fs::File, path::{Path, PathBuf}};

#[derive(Debug, Clone)]
pub struct ParamFile {
    pub path: PathBuf,
}

impl ParamFile {
    pub fn open(path: impl AsRef<Path>) -> Result<Self> {
        let path = path.as_ref().to_path_buf();
        if !path.exists() {
            anyhow::bail!("Parameter file does not exist: {}", path.display());
        }
        Ok(ParamFile { path })
    }

    pub fn map(&self) -> Result<Mmap> {
        let file = File::open(&self.path)
            .with_context(|| format!("Failed to open parameter file {}", self.path.display()))?;
        unsafe {
            MmapOptions::new()
                .map(&file)
                .context("Failed to memory-map parameter file")
        }
    }

    pub fn path(&self) -> &Path {
        &self.path
    }
}