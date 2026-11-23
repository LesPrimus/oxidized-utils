use anyhow::Context;
use clap::Parser;
use std::io::{BufRead, Read};

use crate::cli::Args;
use common::NamedReader;

mod cli;

fn main() {
    if let Err(e) = run(Args::parse()) {
        eprintln!("{e:#}");
        std::process::exit(1);
    }
}

fn run(args: Args) -> anyhow::Result<()> {
    let filenames = args.files;
    let lines = args.lines;

    let should_print_header = filenames.len() > 1;

    for (idx, filename) in filenames.iter().enumerate() {
        match NamedReader::open(filename) {
            Ok(reader) => {
                if should_print_header {
                    print_header(reader.name().unwrap_or("stdin"), idx > 0);
                }

                match args.bytes {
                    Some(n) => read_bytes(reader, n),
                    None => read_lines(reader, lines),
                }
                .with_context(|| format!("failed to read {filename}"))?
            }
            Err(e) => eprintln!("{filename}: {e}"),
        }
    }
    Ok(())
}

fn print_header(name: &str, add_new_line: bool) {
    if add_new_line {
        println!();
    }
    println!("==> {name} <==");
}

fn read_bytes(reader: impl Read, bytes: u64) -> anyhow::Result<()> {
    let mut buffer = vec![];
    reader.take(bytes).read_to_end(&mut buffer)?;
    print!("{}", String::from_utf8_lossy(&buffer));
    Ok(())
}

fn read_lines(mut reader: impl BufRead, lines: u64) -> anyhow::Result<()> {
    let mut buf = String::new();

    for _ in 0..lines {
        buf.clear();
        let bytes = reader.read_line(&mut buf)?;
        if bytes == 0 {
            break;
        }
        print!("{buf}")
    }
    Ok(())
}
