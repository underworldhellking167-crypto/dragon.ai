 use anyhow::{Context, Result};
 use std::{fs, io::Write, path::Path};

pub fn extract_params(from: impl AsRef<Path>, to: impl AsRef<Path>) -> Result<()> {
    let from = from.as_ref();
    let to = to.as_ref();
    let input = fs::read(from)
        .with_context(|| format!("Failed to read source file {}", from.display()))?;
    fs::create_dir_all(to)
        .with_context(|| format!("Failed to create target directory {}", to.display()))?;
    let target = to.join(
        from.file_name()
            .ok_or_else(|| anyhow::anyhow!("Source file has no filename"))?,
    );
    let mut output = fs::File::create(&target)
        .with_context(|| format!("Failed to create target file {}", target.display()))?;
    output.write_all(&input).context("Failed to write extracted parameters")?;
    println!("Extracted params to {}", target.display());
    Ok(())
}