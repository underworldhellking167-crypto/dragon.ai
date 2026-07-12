// 🐉 DRAGON.AI — Auto-Update Checker
// Checks GitHub for new releases

use colored::*;

const GITHUB_API: &str = "https://api.github.com/repos/underworldhellking167-crypto/dragon.ai/releases/latest";
const CURRENT_VERSION: &str = "1.0.0";

pub async fn check_for_updates() {
    println!("🔍 Checking for updates...");
    
    match reqwest::get(GITHUB_API).await {
        Ok(response) => {
            if let Ok(release) = response.json::<serde_json::Value>().await {
                let latest = release["tag_name"].as_str().unwrap_or("unknown");
                
                if latest != CURRENT_VERSION {
                    println!();
                    println!("{}", "╔══════════════════════════════════════════╗".yellow());
                    println!("{}", "║  🆕 NEW VERSION AVAILABLE!              ║".yellow());
                    println!("{}", format!("║  Current: v{} → Latest: {}     ║", CURRENT_VERSION, latest).yellow());
                    println!("{}", "║  Run: git pull && cargo build --release  ║".yellow());
                    println!("{}", "╚══════════════════════════════════════════╝".yellow());
                    println!();
                } else {
                    println!("  ✅ Dragon.AI is up to date (v{})", CURRENT_VERSION);
                }
            }
        }
        Err(_) => {
            // Silently fail — update check is optional
        }
    }
}

pub fn current_version() -> &'static str {
    CURRENT_VERSION
}
