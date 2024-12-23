use clap::Parser;
use std::{
    fs,
    io::{self, BufRead, BufReader, Read},
    process,
};

#[derive(Parser, Debug)]
struct Args {
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
            wc(fs::File::open(path)?)?
        }
        None => wc(std::io::stdin())?,
    }

    Ok(())
}

fn wc<R: Read>(reader: R) -> io::Result<()> {
    let buf_reader = BufReader::new(reader);
    let mut word_count = 0;
    let mut line_count = 0;
    let mut char_count = 0;

    for line in buf_reader.lines() {
        match line {
            Ok(line) => {
                word_count += line.split_whitespace().count();
                line_count += 1;
                char_count += line.len();
            }
            Err(e) => Err(e)?,
        }
    }

    println!("Lines: {}", line_count);
    println!("Words: {}", word_count);
    println!("Chars: {}", char_count);

    Ok(())
}
