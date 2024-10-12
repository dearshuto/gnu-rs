use std::io::{BufRead, BufReader, Read};

use clap::{ArgGroup, Parser};

#[derive(Debug, Parser)]
#[clap(group(ArgGroup::new("aaae")
    .args(&["line_number"])
    .requires_all(&["line_number"])
    .multiple(true)
    .conflicts_with("byte_size"))
)]
struct Args {
    input: String,

    /// print the first NUM bytes of each file
    #[arg(short = 'c', long = "bytes")]
    byte_size: Option<usize>,

    /// print the first NUM lines instead of the first 10
    #[arg(short = 'n', long = "lines", default_value = "10")]
    line_number: u32,
}

fn run_lines<T>(read: T, line_number: u32, byte_size: Option<usize>)
where
    T: Read,
{
    let reader = BufReader::new(read);

    if let Some(byte_size) = byte_size {
        let data: Vec<_> = reader.bytes().map(|x| x.unwrap()).take(byte_size).collect();
        let str = String::from_utf8(data).unwrap();
        println!("{}", str);
    } else {
        for line in reader.take(line_number as u64).lines() {
            let Ok(line) = line else {
                break;
            };

            println!("{line}");
        }
    }
}

fn main() {
    let args = Args::parse();

    if args.input == "-" {
        run_lines(std::io::stdin(), args.line_number, args.byte_size);
    } else {
        run_lines(
            std::fs::File::open(&args.input).unwrap(),
            args.line_number,
            args.byte_size,
        );
    }
}
