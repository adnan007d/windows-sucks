use clap::Parser;
use std::{
    fs,
    io::{self, Read, Seek, Write},
    os::windows::fs::MetadataExt,
    process,
};

const MAX_CHUNK_SIZE: usize = 14;

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
            tail(path, metadata.file_size(), args.number)?
        }
        None => {
            eprintln!("Not Implemented for stdin");
            process::exit(1);
        }
    }

    Ok(())
}

fn tail(path: String, file_size: u64, n: usize) -> io::Result<()> {
    let mut file = fs::File::open(path)?;

    let mut current_pos = file_size;
    let mut buf: Vec<u8> = Vec::new();
    buf.reserve_exact(MAX_CHUNK_SIZE);

    let mut leftover_chunk: Vec<u8> = Vec::new();

    let mut output: Vec<Vec<u8>> = Vec::new();
    output.reserve_exact(n);

    let _ = file.seek(std::io::SeekFrom::End(0));

    while current_pos > 0 {
        let chunk_size = std::cmp::min(current_pos, MAX_CHUNK_SIZE as u64) as i64;
        buf.clear();
        buf.resize(chunk_size as usize, 0);

        _ = file.seek(std::io::SeekFrom::Current(-chunk_size))?;
        // println!("{}", current_pos);
        file.read_exact(&mut buf)?;
        current_pos = file.seek(std::io::SeekFrom::Current(-chunk_size))?;

        buf.extend(&leftover_chunk);
        let mut prev_pos = buf.len();
        for (i, c) in buf.iter().enumerate().rev() {
            // Skip first new line
            if i == 0 {
                continue;
            }

            // Required lines collected
            if output.len() == n {
                break;
            }

            if *c == b'\n' {
                output.push(buf[i + 1..prev_pos].to_vec());
                prev_pos = i + 1;
            }
        }
        if current_pos == 0 {
            output.push(buf[..prev_pos].to_vec());
        } else {
            leftover_chunk = buf[..prev_pos].to_vec()
        }
    }

    for ele in output.iter().rev() {
        if let Err(e) = std::io::stdout().write_all(ele) {
            eprintln!("Error while writing to stdout {}", e);
            break;
        }
    }

    Ok(())
}
