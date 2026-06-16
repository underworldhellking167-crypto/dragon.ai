use anyhow::Result;
use std::path::Path;
 use walkdir::WalkDir;

const SUPPORTED_EXTENSIONS: &[&str] = &[
    "params",
    "gguf",
    "safetensors",
    "bin",
];

pub fn scan_params(path: impl AsRef<Path>) -> Result<Vec<String>> {
    let path = path.as_ref();
    let mut results = Vec::new();

    if !path.exists() {
        anyhow::bail!("Scan path does not exist: {}", path.display());
    }

    for entry in WalkDir::new(path).into_iter().filter_map(|e| e.ok()) {
        let path = entry.path();
        if !path.is_file() {
            continue;
        }
        if let Some(ext) = path.extension().and_then(|e| e.to_str()) {
            if SUPPORTED_EXTENSIONS.contains(&ext) {
                results.push(path.display().to_string());
            }
        }
    }

    Ok(results)
}

pub fn scan_default_params() -> Result<Vec<String>> {
    scan_params("params")
}