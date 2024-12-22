use clap::Parser;
use std::{
    fs,
    io::{self, Read, Seek, Write},
    process,
};

// 16 Kb
const MAX_CHUNK_SIZE: usize = 16 * 1024;

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
            tail(path, args.number)?
        }
        None => {
            eprintln!("Not Implemented for stdin");
            process::exit(1);
        }
    }

    Ok(())
}

fn tail(path: String, n: usize) -> io::Result<()> {
    let mut file = std::fs::File::open(path)?;

    // Go to end and it will return the file size
    let mut file_size = file.seek(std::io::SeekFrom::End(0))?;

    // Buffer to store the bytes
    let mut buf: Vec<u8> = vec![0; MAX_CHUNK_SIZE];

    // To store the bytes that are yet to encounter the new line character
    let mut leftover_chunk: Vec<u8> = Vec::new();

    // To store the lines
    let mut lines: Vec<Vec<u8>> = Vec::with_capacity(n);

    while file_size > 0 && lines.len() < n {
        let chunk_size = std::cmp::min(MAX_CHUNK_SIZE, file_size as usize);
        file_size -= chunk_size as u64;

        file.seek(std::io::SeekFrom::Start(file_size))?;
        file.read_exact(&mut buf[..chunk_size])?;

        // Creating a new buffer to store
        let mut combined_chunk = Vec::with_capacity(chunk_size + leftover_chunk.len());
        combined_chunk.extend_from_slice(&buf[..chunk_size]);
        combined_chunk.extend_from_slice(&leftover_chunk);

        let mut prev_pos = combined_chunk.len();
        for (i, ch) in combined_chunk.iter().enumerate().rev() {

            if i == combined_chunk.len() - 1 {
                continue;
            }

            if lines.len() == n {
                break;
            }

            if *ch == b'\n' {
                lines.push(combined_chunk[i + 1..prev_pos].to_vec());
                prev_pos = i;
            }
        }

        leftover_chunk = combined_chunk[..prev_pos].to_vec();
    }

    if file_size == 0 {
        lines.push(leftover_chunk);
    }

    let stdout = std::io::stdout();
    let mut handle = stdout.lock();
    for line in lines.iter().rev() {
        handle.write_all(line)?;
        handle.write_all(b"\n")?;
    }

    Ok(())
}
