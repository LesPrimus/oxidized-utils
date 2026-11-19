use clap::Parser;

#[derive(Debug, Parser)]
#[command(version, about)]
pub struct Args {
    #[arg(value_name = "FILE", default_value = "-")]
    pub files: Vec<String>,

    #[arg(
        short = 'n',
        long,
        default_value_t = 10,
        conflicts_with = "bytes",
        help = "Number of lines",
        value_parser = clap::value_parser!(u64).range(1..),
    )]
    pub lines: u64,

    #[arg(
        short = 'c',
        long,
        help = "Number of bytes",
        value_parser = clap::value_parser!(u64).range(1..)
    )]
    pub bytes: Option<u64>,
}
