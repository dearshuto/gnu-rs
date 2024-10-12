use std::io::{BufRead, BufReader, Read};

use clap::Parser;

#[derive(Debug, Parser)]
struct Args {
    input: String,

    /// print the first NUM bytes of each file
    #[arg(short = 'c', long = "bytes")]
    byte_size: usize,

    /// print the first NUM lines instead of the first 10
    #[arg(short = 'n', long = "lines")]
    line_number: u32,
}

fn main() {
    let _args = Args::parse();
    println!("Hello, world!");
}
