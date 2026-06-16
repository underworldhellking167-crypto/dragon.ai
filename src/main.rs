 use clap::{Parser, Subcommand};
 use colored::*;
 use std::path::PathBuf;

 mod api;
 mod architecture;
 mod config;
 mod extractor;
 mod gpu;
 mod inference;
 mod params;
 mod scanner;
 mod tui;

 use config::Config;
use extractor::extract_params;
use scanner::scan_default_params;
 use tui::show_menu;

#[derive(Parser)]
#[command(name = "dragon", author, version, about = "Terminal-based universal AI runtime engine", long_about = None)]
struct Cli {
    #[command(subcommand)]
    command: Option<Commands>,
}

#[derive(Subcommand)]
enum Commands {
    Run {
        #[arg(long)]
        params: PathBuf,
        #[arg(long)]
        context: Option<usize>,
    },
    Serve {
        #[arg(long)]
        params: PathBuf,
        #[arg(long, default_value_t = 8080)]
        port: u16,
    },
    Scan,
    Extract {
        #[arg(long)]
        from: PathBuf,
        #[arg(long)]
        to: PathBuf,
    },
    Config,
    Ask {
        #[arg(long)]
        question: String,
        #[arg(long)]
        params: PathBuf,
    },
}

fn print_banner() {
    println!("{}", "Dragon.AI — Terminal Universal AI Runtime".bold().green());
    println!("{}", "Your engine runs any parameter file.".italic().yellow());
    println!();
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    print_banner();
    let cli = Cli::parse();
    let config = Config::load()?;

    match cli.command {
        Some(Commands::Run { params, context }) => {
            let context_len = context.unwrap_or(config.context_len);
            println!("Would run with params: {}", params.display());
            println!("Would use context length: {}", context_len);
        }
        Some(Commands::Serve { params, port }) => {
            println!("Would serve API with params: {}", params.display());
            println!("Would listen on port: {}", port);
        }
        Some(Commands::Scan) => {
            println!("Scanning... (stubbed, will walk params directories and detect .params / .gguf / .safetensors)");
            let results = scan_default_params()?;
            println!("Found {} files", results.len());
        }
        Some(Commands::Extract { from, to }) => {
            println!("Would extract params from {} to {}", from.display(), to.display());
            extract_params(from, to)?;
        }
        Some(Commands::Config) => {
            config.print()?;
        }
        Some(Commands::Ask { question, params }) => {
            println!("Would ask question: {}", question);
            println!("Would use params: {}", params.display());
        }
        None => {
            println!("Interactive mode coming soon.");
            show_menu().await?;
        }
    }

    Ok(())
}