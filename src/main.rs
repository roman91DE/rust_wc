use std::fs::File;
use std::io::{BufRead, BufReader, Error, ErrorKind};
use std::path::Path;

struct Metrics {
    chars: u128,
    words: u128,
    lines: u128,
}

fn str_to_path(s: &str) -> &Path {
    Path::new(s)
}

fn file_exists(path: &Path) -> bool {
    path.exists() && path.is_file()
}

fn open_file(path: &Path) -> Result<File, Error> {
    if !file_exists(path) {
        return Err(Error::new(
            ErrorKind::NotFound,
            format!("File not found: {}", path.display()),
        ));
    }
    File::open(path)
}

fn get_metrics(f: &File) -> Result<Metrics, Error> {
    let mut chars = 0u128;
    let mut words = 0u128;
    let mut lines = 0u128;

    let reader = BufReader::new(f);

    for (line_number, line_result) in reader.lines().enumerate() {
        let line = line_result.expect(&format!("Error reading line {}", line_number + 1));

        lines += 1;
        chars += line.len() as u128 + 1;
        words += line.split_whitespace().count() as u128;
    }

    Ok(Metrics {
        chars,
        words,
        lines,
    })
}

fn main() -> Result<(), Error> {
    let args: Vec<String> = std::env::args().skip(1).collect();

    if args.is_empty() {
        eprintln!("No files provided. Usage: program <file1> <file2> ...");
        return Ok(());
    }

    for arg in args {
        let path = str_to_path(&arg);

        let file = match open_file(path) {
            Ok(file) => file,
            Err(err) => {
                eprintln!("Couldn't open file '{}': {}", arg, err);
                continue; // Skip to the next file
            }
        };

        match get_metrics(&file) {
            Ok(metrics) => {
                println!("File: {}", arg);
                print!(
                    "\t{:?}\t{:?}\t{:?} {}\n",
                    metrics.lines, metrics.words, metrics.chars, arg
                );
            }
            Err(err) => {
                eprintln!("Error processing file '{}': {}", arg, err);
            }
        }
    }

    Ok(())
}
