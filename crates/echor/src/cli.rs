use clap::Parser;

#[derive(Debug, Parser)]
#[command(version, about)]
pub struct Args {
    #[arg(required = true)]
    pub text: Vec<String>,

    #[arg(short = 'n')]
    pub omit_newline: bool,
}
