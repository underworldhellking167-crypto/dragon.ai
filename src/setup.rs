// 🐉 DRAGON.AI — First-Run Setup Wizard

use crate::config::Config;
use colored::*;
use std::io::{self, Write};

pub fn run_setup() -> Config {
    println!();
    println!("{}", "╔══════════════════════════════════════════════╗".red());
    println!("{}", "║     🐉 DRAGON.AI — First Run Setup          ║".red());
    println!("{}", "╚══════════════════════════════════════════════╝".red());
    println!();
    
    let mut config = Config::default();
    
    // Question 1: Do you have a model?
    println!("{}", "📋 STEP 1: Find a Brain".yellow().bold());
    println!("Do you have a .gguf model file?");
    println!("  [1] Yes, I have one");
    println!("  [2] No, scan my system");
    println!("  [3] Skip for now");
    print!("> ");
    io::stdout().flush().unwrap();
    
    let mut choice = String::new();
    io::stdin().read_line(&mut choice).unwrap();
    
    match choice.trim() {
        "1" => {
            print!("Enter path to .gguf file: ");
            io::stdout().flush().unwrap();
            let mut path = String::new();
            io::stdin().read_line(&mut path).unwrap();
            config.params = Some(path.trim().to_string());
        }
        "2" => {
            println!("\n🔍 Scanning for .gguf files...");
            crate::scanner::scan_for_brains();
            print!("\nEnter path from above (or press Enter to skip): ");
            io::stdout().flush().unwrap();
            let mut path = String::new();
            io::stdin().read_line(&mut path).unwrap();
            if !path.trim().is_empty() {
                config.params = Some(path.trim().to_string());
            }
        }
        _ => println!("  Skipping."),
    }
    
    // Question 2: Context length
    println!();
    println!("{}", "📋 STEP 2: Context Length".yellow().bold());
    println!("How many tokens of memory? (default: 32768)");
    println!("  [1] 4096   (short)");
    println!("  [2] 8192   (medium)");
    println!("  [3] 32768  (long — recommended)");
    println!("  [4] 128000 (very long)");
    print!("> ");
    io::stdout().flush().unwrap();
    
    choice.clear();
    io::stdin().read_line(&mut choice).unwrap();
    config.context_length = match choice.trim() {
        "1" => 4096,
        "2" => 8192,
        "4" => 128000,
        _ => 32768,
    };
    
    // Question 3: CPU Threads
    println!();
    println!("{}", "📋 STEP 3: CPU Threads".yellow().bold());
    let cores = num_cpus::get();
    println!("Detected {} CPU cores.", cores);
    println!("How many to use? (default: {})", cores / 2);
    print!("> ");
    io::stdout().flush().unwrap();
    
    choice.clear();
    io::stdin().read_line(&mut choice).unwrap();
    if let Ok(n) = choice.trim().parse::<u32>() {
        config.cpu_threads = n;
    } else {
        config.cpu_threads = (cores / 2) as u32;
    }
    
    // Question 4: GPU
    println!();
    println!("{}", "📋 STEP 4: GPU Acceleration".yellow().bold());
    println!("Use GPU? (default: No)");
    println!("  [1] Yes");
    println!("  [2] No");
    print!("> ");
    io::stdout().flush().unwrap();
    
    choice.clear();
    io::stdin().read_line(&mut choice).unwrap();
    config.gpu_enabled = choice.trim() == "1";
    
    // Save
    crate::config::save(&config);
    
    println!();
    println!("{}", "✅ Setup complete!".green().bold());
    println!("  Config saved to: ~/.dragon/config.yml");
    println!("  Run: dragon run   to start!");
    println!();
    
    config
}
