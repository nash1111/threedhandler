#![allow(unused)]

use clap::Parser;
use std::io::BufReader;
use std::fs::File;
use std::io::BufRead;

/// Search for a pattern in a file and display the lines that contain it.
#[derive(Parser)]
struct Cli {
    /// The pattern to look for
    pattern: String,
    /// The path to the file to read
    path: std::path::PathBuf,
}

fn main() {
    let args = Cli::parse();

    let content = File::open(&args.path).expect("could not read file");
    let mut reader = BufReader::new(content);
    let mut line = String::new();

    for line in reader.lines() {
        if line.as_ref().unwrap().contains(&args.pattern) {
            println!("{}", line.unwrap());
        }
    }
    println!("path = {:?}", args.path);
    println!("pattern = {:?}", args.pattern);
}