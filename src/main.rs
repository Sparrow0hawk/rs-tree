#![allow(unused)]
use clap::Parser;
use std::fs;
use std::fs::{DirEntry, ReadDir};
use std::io;
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
    // these line below do the real magic
    // first we specify a variable paths which will be a vector containing
    // a result struct of a DirEntry or an Error (similar to ReadDir struct)
    let mut paths: Vec<Result<DirEntry, std::io::Error>>;
    // call read_dir on the path returns a ReadDir iterator
    paths = fs::read_dir(path)
        // unwrap ReadDir iterator into io::Result<DirEntry> or io::Error
        .unwrap()
        // become iterator of Result<DirEntry>
        .into_iter()
        // filter items in iterator checking whether DirEntry is hidden
        // is_hidden returns true is filename starts with .
        // invert boolean using !
        // we pass function a reference and use as_ref to specify that reference
        // is applied to internally Option<&_> rather than &Option<_>
        .filter(|e| !is_hidden(e.as_ref().unwrap()))
        // collect result into vector
        .collect();

    let indent = String::from(" ");

    for path in paths {
        let path_result = path.unwrap();

        let path_comp: usize = path_result.path().components().count();

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

fn is_hidden(entry: &DirEntry) -> bool {
    // taken from https://docs.rs/walkdir/latest/walkdir/
    entry
        .file_name()
        .to_str()
        .map(|item| item.starts_with("."))
        .unwrap_or(false)
}
