mod bootsector;
mod mbr;
mod pbrfat;

use bootsector::{BootSectorKind, infer};
use std::io::prelude::Read;
use std::fs::File;
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
    let path = args.infile;
    let mut f = File::open(path).expect("File not found");
    let mut buf = [0u8; 512];
    f.read_exact(&mut buf).expect("Failed file read");
    match infer(&buf) {
        BootSectorKind::MBR => {
            unimplemented!()
        },
        BootSectorKind::PBRFat => {
            unimplemented!()
        },
        BootSectorKind::Unknown => {
            eprintln!("Unknown bootsector type");
        },
    };
}
