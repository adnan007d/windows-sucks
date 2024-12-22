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
            let metadata = fs::metadata(&path)?;
            // Shouldn't be a directory
            if metadata.is_dir() {
                eprintln!("{0} is a directory", path);
                process::exit(1);
            }
            head(fs::File::open(path)?, args.number)?
        }
        None => head(std::io::stdin(), args.number)?,
    }

    Ok(())
}

fn head<R: Read>(reader: R, n: usize) -> io::Result<()> {
    let buf_reader = BufReader::new(reader);
    for line in buf_reader.lines().take(n) {
        match line {
            Ok(line) => {
                std::io::stdout().write_all(line.as_bytes())?;
                std::io::stdout().write_all(b"\n")?;
            }
            Err(e) => Err(e)?,
        }
    }

    std::io::stdout().flush()?;

    Ok(())
}
