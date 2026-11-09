use std::path::PathBuf;

use clap::{Parser, Subcommand};
use dexsentry_core::analyze;

#[derive(Debug, Parser)]
#[command(name = "dexsentry", about = "Memory-safe Android APK static analyzer")]
struct Cli {
    #[command(subcommand)]
    command: Command,
}

#[derive(Debug, Subcommand)]
enum Command {
    Scan { file: PathBuf },
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    match Cli::parse().command {
        Command::Scan { file } => {
            let report = analyze(&file)?;
            println!("{report:#?}");
        }
    }
    Ok(())
}
