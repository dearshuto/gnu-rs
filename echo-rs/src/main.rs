use clap::Parser;

#[derive(Debug, Parser)]
struct Args {
    input: String,
}

fn main() {
    let args = Args::parse();
    println!("{}", args.input);
}
