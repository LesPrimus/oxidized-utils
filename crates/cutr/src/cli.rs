use clap::{self, Parser};

#[derive(Debug, Parser)]
pub struct Args {
    #[arg(default_value = "-")]
    pub files: Vec<String>,

    #[arg(short, long = "delim", default_value = "\t")]
    pub delimiter: String,

    #[command(flatten)]
    pub extract: ArgsExtract,
}

#[derive(Debug, Clone, clap::Args)]
#[group(required = true, multiple = false)]
pub struct  ArgsExtract {
    #[arg(short, long)]
    pub fields: Option<String>,

    #[arg(short, long)]
    pub bytes: Option<String>,

    #[arg(short, long)]
    pub chars: Option<String>,
}