mod bootsector;

use std::path::PathBuf;
use clap::Parser;

/// Analyze and display MBR, PBR informations
#[derive(Parser)]
#[command(version, about, long_about = None)]
struct Args {
    /// Input file path to analyze file maybe included MBR/PBR.
    #[arg(value_name="FILE")]
    infile: PathBuf,
}

fn main() {
    let args = Args::parse();
    println!("filename: {}", args.infile.display());
}
