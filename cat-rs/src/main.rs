use std::io::{BufRead, BufReader, Read};

use clap::Parser;

#[derive(Debug, Parser)]
struct Args {
    input: String,

    /// number all output lines
    #[arg(short = 'n', long = "number")]
    is_numbered: bool,
}

fn run<T>(backend: T, is_numbered: bool)
where
    T: Read,
{
    let mut line_number = 0;
    let reader = BufReader::new(backend);
    for line in reader.lines() {
        let Ok(line) = line else {
            break;
        };

        if is_numbered {
            println!("{line_number} {line}");
            line_number += 1;
        } else {
            println!("{line}");
        }
    }
}

fn main() {
    let args = Args::parse();

    // run stdin mode when - is detected
    if args.input == "-" {
        run(std::io::stdin(), args.is_numbered);
        return;
    }

    let path = std::path::Path::new(&args.input);
    if !path.exists() {
        println!("{:?}: No such file or directory", path);
        return;
    }

    let Ok(file) = std::fs::File::open(path) else {
        return;
    };

    run(file, args.is_numbered);
}
