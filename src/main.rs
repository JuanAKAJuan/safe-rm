use clap::Parser;
use std::fs;
use std::io::ErrorKind;
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

    for path in args.paths {
        match fs::metadata(&path) {
            Ok(_) => {
                // Path exists, add moving to trash logic here
                println!("Path exists {:?}", path);
            }
            Err(error) => {
                if error.kind() == ErrorKind::NotFound {
                    eprintln!("Error: Path not found: {:?}", path);
                } else {
                    eprintln!("Error: {:?}", error);
                }
            }
        }
    }
}
