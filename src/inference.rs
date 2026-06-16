 use anyhow::Result;
 use crate::params::ParamFile;

pub async fn run_inference(params: &ParamFile, context_len: usize) -> Result<()> {
    println!("Would run inference with {}", params.path().display());
    println!("Would use context length: {} tokens", context_len);
    println!("(Placeholder inference engine)");
    Ok(())
}