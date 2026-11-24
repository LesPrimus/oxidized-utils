use clap::Parser;

#[derive(Debug, Parser)]
pub struct Args {
    #[arg(default_value = "-")]
    pub in_file: String,

    pub out_file: Option<String>,

    #[arg(short, long)]
    pub count: bool,
}
