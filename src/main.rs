use clap::Parser;
use std::path::PathBuf;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    // Files or directories to remove
    #[clap(value_parser)]
    paths: Vec<PathBuf>,
}

fn main() {
    let args = Args::parse();
    println!("Paths: {:?}", args.paths);

    for path in args.paths {
        println!("Path: {:?}", path);
    }
}
