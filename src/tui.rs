 use anyhow::Result;

pub async fn show_menu() -> Result<()> {
    println!("\nDragon.AI Terminal Menu");
    println!("1) Config");
    println!("2) Params");
    println!("3) Settings");
    println!("4) Infer");
    println!("5) Exit");
    Ok(())
}