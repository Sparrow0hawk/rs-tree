#![allow(unused)]
use clap::Parser;
use std::fs;
use std::mem;

#[derive(Parser, Debug)]
struct Cli {
    // path to create tree from
    #[clap(parse(from_os_str))]
    path: std::path::PathBuf,
}

fn main() {
    let args = Cli::parse();

    walk_dirs(args.path);
}

fn walk_dirs(path: std::path::PathBuf) {
    let mut paths = fs::read_dir(path).unwrap();

    for path in paths {
        let path_result = path.unwrap();
        println!("{}", path_result.path().display());
        if fs::metadata(path_result.path()).unwrap().is_dir() {
            walk_dirs(path_result.path());
        }
    }
}
