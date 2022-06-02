#![allow(unused)]
use clap::Parser;

#[derive(Parser)]
struct Cli {
    // path to create tree from
    #[clap(parse(from_os_str))]
    path: std::path::PathBuf,
}

fn main() {
    let args = Cli::parse();
}
