#![allow(unused)]

extern crate stl;
use stl::{ read_ascii_stl, ParsedSTL };

use clap::Parser;
use std::io::BufReader;
use std::fs::File;
use std::fs;
use std::io::BufRead;
use std::string;

/// Search for a pattern in a file and display the lines that contain it.
#[derive(Parser)]
struct Cli {
    /// The pattern to look for
    pattern: String,
    /// The path to the file to read
    path: std::path::PathBuf,
}

enum IOType {
    ReadAsciiSTL,
    WriteAsciiSTL,
}

fn args_to_io_type(pattern: String) -> IOType {
    let read_ascii_stl_pattern = "read_ascii_stl";
    let write_ascii_stl_pattern = "write_ascii_stl";
    if pattern == read_ascii_stl_pattern {
        return IOType::ReadAsciiSTL;
    }
    panic!("Unknown file type");
}

fn main() {
    let args = Cli::parse();

    let content = fs::read_to_string(&args.path).expect("could not read file");
    //let mut reader = BufReader::new(content);
    //let mut line = String::new();
    //let read_ascii_stl = IOType::ReadAsciiSTL;

    // argsを使ってmatch式で分岐させる
    // args.pathの値をenumに落とし込む

    let pattern = args.pattern;
    let io_type = args_to_io_type(pattern);
    stl::add(1, 3);
    let mut parsed_stl: ParsedSTL;

    match io_type {
        IOType::ReadAsciiSTL => parsed_stl = read_ascii_stl(content),
        // todo
        // call readasciiSTL
        IOType::WriteAsciiSTL => parsed_stl = read_ascii_stl(content),
    }
}