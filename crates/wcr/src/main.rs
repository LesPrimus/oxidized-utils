use clap::Parser;

use crate::cli::Args;

mod cli;
mod file_info;

fn main() {
    if let Err(e) = run(Args::parse()) {
        eprintln!("{e:#}");
        std::process::exit(1);
    }
}

fn normalize_cli_args(mut args: Args) -> Args {
    if !(args.lines || args.words || args.bytes || args.chars) {
        args.lines = true;
        args.words = true;
        args.bytes = true;
    }
    args
}

fn run(args: Args) -> anyhow::Result<()> {
    let args = normalize_cli_args(args);
    dbg!(args);
    Ok(())
}
