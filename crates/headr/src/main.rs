use clap::Parser;

use crate::cli::Args;

mod cli;

fn main() {
    if let Err(e) = run(cli::Args::parse()) {
        eprintln!("{e}");
        std::process::exit(1);
    }
}

fn run(args: Args) -> anyhow::Result<()> {
    Ok(())
}
