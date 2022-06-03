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
    /// recursive walk directories function
    ///
    /// prints directory
    let mut paths = fs::read_dir(path).unwrap();

    let indent = String::from(" ");

    for path in paths {
        let path_result = path.unwrap();

        let path_comp = path_result.path().components().count();

        if fs::metadata(path_result.path()).unwrap().is_dir() {
            println!(
                "{}{}/",
                indent.repeat(path_comp),
                path_result.file_name().to_str().unwrap()
            );

            walk_dirs(path_result.path());
        } else {
            println!(
                "{}{}",
                indent.repeat(path_comp + 2),
                path_result.file_name().to_str().unwrap()
            );
        }
    }
}
