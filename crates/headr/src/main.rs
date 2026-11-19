use clap::Parser;
use std::{
    fs::File,
    io::{BufRead, BufReader},
};

use crate::cli::Args;

mod cli;

fn main() {
    if let Err(e) = run(Args::parse()) {
        eprintln!("{e}");
        std::process::exit(1);
    }
}

fn run(args: Args) -> anyhow::Result<()> {
    Ok(())
}

fn open(filename: &str) -> anyhow::Result<Box<dyn BufRead>> {
    let buffer: Box<dyn BufRead> = match filename {
        "-" => Box::new(BufReader::new(std::io::stdin())),
        _ => Box::new(BufReader::new(File::open(filename)?)),
    };
    Ok(buffer)
}
