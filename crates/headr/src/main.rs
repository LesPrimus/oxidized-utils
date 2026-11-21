use anyhow::Context;
use clap::Parser;
use std::io::BufRead;

use crate::{cli::Args, named_reader::NamedReader};

mod cli;
mod named_reader;

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
                    print_header(reader.name(), idx > 0);
                }
                read(reader, lines).with_context(|| format!("failed to read {filename}"))?;
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

fn read(mut reader: impl BufRead, lines: u64) -> anyhow::Result<()> {
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
