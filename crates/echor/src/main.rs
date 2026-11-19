use clap::Parser;

mod cli;

fn main() {
    let args = cli::Args::parse();
    let ending = if args.omit_newline { "" } else { "\n" };

    print!("{}{ending}", args.text.join(" "));
}
