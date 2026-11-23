use clap::Parser;

mod cli;
mod file_info;

fn main() {
    let args = cli::Args::parse();
    dbg!(args);
}
