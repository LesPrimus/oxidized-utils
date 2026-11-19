use clap::Parser;

#[derive(Debug, Parser)]
#[command(version, about)]
pub struct Args {
    #[arg(value_name = "FILE", default_value = "-")]
    pub files: Vec<String>,

    #[arg(short = 'n', long = "number")]
    pub number_lines: bool,

    #[arg(short = 'b', long = "number-nonblank", conflicts_with = "number_lines")]
    pub number_nonblank_lines: bool,
}
