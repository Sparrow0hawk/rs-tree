#![allow(unused)]
use clap::Parser;
use std::fs;

#[derive(Parser, Debug)]
struct Cli {
    // path to create tree from
    #[clap(parse(from_os_str))]
    path: std::path::PathBuf,
}

fn main() {
    let args = Cli::parse();

    let paths = fs::read_dir(&args.path).unwrap();

    for path in paths {
        let path_result = path.unwrap();

        println!("{}", path_result.path().display());

        if fs::metadata(path_result.path()).unwrap().is_dir() {
            for sub_path in fs::read_dir(path_result.path()).unwrap() {
                println!(" -- {}", sub_path.unwrap().path().display())
            }
        }
    }
}
