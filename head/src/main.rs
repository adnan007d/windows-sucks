use clap::Parser;
use std::{
    fs,
    io::{self, BufRead, BufReader, Read, Write},
    process,
};

#[derive(Parser, Debug)]
struct Args {
    #[arg(short, long)]
    #[clap(default_value_t = 10)]
    number: usize,
    path: Option<String>,
}

fn main() -> io::Result<()> {
    let args = Args::parse();

    match args.path {
        Some(path) => {
            // Check if file exists
            let metadata = fs::metadata(&path).unwrap_or_else(|_| {
                eprintln!("{0} does not exists", path);
                process::exit(1);
            });

            // Shouldn't be a directory
            if metadata.is_dir() {
                eprintln!("{0} is a directory", path);
                process::exit(1);
            }
            head(fs::File::open(path)?, args.number)
        }
        None => head(std::io::stdin(), args.number),
    }

    Ok(())
}

fn head<R: Read>(reader: R, n: usize) {
    let mut buf = String::new();
    let mut buf_reader = BufReader::new(reader);
    for _ in 0..n {
        buf.clear();
        match buf_reader.read_line(&mut buf) {
            Ok(0) => break,
            Ok(_) => {
                if let Err(e) = std::io::stdout().write_all(buf.as_bytes()) {
                    eprintln!("Error while writing to stdout {}", e);
                    break;
                }
            }
            Err(e) => {
                eprintln!("Error reading from input {}", e);
                break;
            }
        }
    }

    if let Err(e) = std::io::stdout().flush() {
        eprintln!("Error flushing stdout: {}", e);
    }
}
