use clap::Parser;

#[derive(Debug, Parser)]
#[command(version, about)]
pub struct Args {
    #[arg(default_value = "-", value_name = "FILE")]
    pub files: Vec<String>,

    /// Show the line count.
    #[arg(short, long)]
    pub lines: bool,

    /// Show how many words.
    #[arg(short, long)]
    pub words: bool,

    /// Show how many bytes.
    #[arg(short = 'c', long)]
    pub bytes: bool,

    /// Show how many chars.
    #[arg(short = 'm', long, conflicts_with = "bytes")]
    pub chars: bool,
}
