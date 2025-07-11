// src/main.rs
/*
 * Main executable for BlockchainNFTRegistryPlatformX
 */

use clap::Parser;
use blockchainnftregistryplatformx::{Result, run};

#[derive(Parser)]
#[command(version, about = "BlockchainNFTRegistryPlatformX - A Rust implementation")]
struct Cli {
    /// Enable verbose output
    #[arg(short, long)]
    verbose: bool,
}

fn main() -> Result<()> {
    let args = Cli::parse();
    run(args.verbose)
}
