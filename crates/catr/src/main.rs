use std::fs::File;
use std::io::{self, BufRead, BufReader};

use clap::Parser;

use crate::cli::Args;

mod cli;

fn main() {
    if let Err(e) = run(Args::parse()) {
        eprintln!("{e}");
        std::process::exit(1);
    }
}

fn run(args: Args) -> std::io::Result<()> {
    for filename in args.files {
        match open(&filename) {
            Ok(r) => {
                read(r, args.number_lines, args.number_nonblank_lines)?;
            }
            Err(e) => eprintln!("{filename}: {e}"),
        }
    }
    Ok(())
}

fn open(filename: &str) -> std::io::Result<Box<dyn BufRead>> {
    match filename {
        "-" => Ok(Box::new(BufReader::new(io::stdin()))),
        _ => Ok(Box::new(BufReader::new(File::open(filename)?))),
    }
}

fn read<F: BufRead>(input: F, line_number: bool, number_non_blank: bool) -> std::io::Result<()> {
    let mut counter: usize = 1;

    for line in input.lines() {
        let line = line?;

        if number_non_blank {
            if line.is_empty() {
                println!();
            } else {
                println!("{counter:>6}\t{line}");
                counter += 1;
            }
        } else if line_number {
            println!("{counter:>6}\t{line}");
            counter += 1;
        } else {
            println!("{line}")
        }
    }
    Ok(())
}
