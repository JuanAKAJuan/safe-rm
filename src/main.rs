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

    #[clap(short = 'r', long = "recursive")]
    recursive: bool,

    #[clap(short = 'f', long = "force")]
    force: bool,

    #[clap(short = 'v', long = "verbose", help = "Enable verbose output")]
    verbose: bool,
}

fn main() {
    let args = Args::parse();

    for path in &args.paths {
        match fs::metadata(path) {
            Ok(metadata) => {
                if metadata.is_dir() && !args.recursive {
                    eprintln!(
                        "Cannot remove directory '{}' without -r flag",
                        path.display()
                    );
                    continue;
                }

                match move_to_trash(path, args.recursive, args.force, args.verbose) {
                    Ok(_) => {}
                    Err(error) => {
                        eprintln!("Error moving to trash: '{}': {}", path.display(), error)
                    }
                }
            }
            Err(error) => {
                if error.kind() == ErrorKind::NotFound {
                    if args.force {
                        // With the force flag, silently ignore non-existent files.
                        continue;
                    } else {
                        eprintln!("Path not found: '{}'", path.display());
                    }
                } else {
                    eprintln!("Error: '{}': {}", path.display(), error);
                }
            }
        }
    }
}

fn move_to_trash(
    path: &PathBuf,
    recursive: bool,
    force: bool,
    verbose: bool,
) -> std::io::Result<()> {
    let metadata = match fs::metadata(path) {
        Ok(meta) => meta,
        Err(error) => {
            // When using -f (force), it's okay if the file is not found
            if force && error.kind() == ErrorKind::NotFound {
                return Ok(());
            }
            return Err(error);
        }
    };

    if metadata.is_dir() && !recursive {
        return Err(std::io::Error::new(
            ErrorKind::PermissionDenied,
            "Cannot remove directory without -r flag",
        ));
    }

    let directory_entries = if verbose && metadata.is_dir() {
        let mut entries: Vec<(PathBuf, String)> = Vec::new();
        if let Ok(read_directory) = fs::read_dir(path) {
            for entry_result in read_directory {
                match entry_result {
                    Ok(entry) => {
                        if let Ok(metadata) = entry.metadata() {
                            let file_type = if metadata.is_dir() {
                                "directory"
                            } else {
                                "file"
                            };
                            entries.push((entry.path(), file_type.to_string()));
                        }
                    }
                    Err(_) if force => continue,
                    Err(error) => return Err(error),
                }
            }
        }
        Some(entries)
    } else {
        None
    };

    match trash::delete(path) {
        Ok(_) => {
            if verbose {
                if metadata.is_dir() {
                    println!("Moved directory to trash: '{}'", path.display());

                    if let Some(entries) = directory_entries {
                        for (entry_path, file_type) in entries {
                            println!("  Trashed: '{}' ({})", entry_path.display(), file_type);
                        }
                    }
                } else {
                    // In normal `rm -r`, not specifying a directory and only a file will still delete
                    // that file.
                    let file_size = metadata.len();
                    println!(
                        "Moved file to trash: '{}' (size: {} bytes)",
                        path.display(),
                        file_size
                    );
                }
            }
            Ok(())
        }
        Err(error) => {
            // Convert the trash crate error to an io::Error
            Err(std::io::Error::new(
                ErrorKind::Other,
                format!("Failed to move to trash: '{}'", error),
            ))
        }
    }
}
