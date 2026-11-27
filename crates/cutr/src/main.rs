use crate::cli::{Args, ArgsExtract};
use clap::Parser;
use common::parsers::{PositionList, parse_range};

mod cli;

pub enum ExtractedParam {
    Fields(PositionList),
    Bytes(PositionList),
    Chars(PositionList),
}

fn main() {
    if let Err(e) = run(Args::parse()) {
        eprintln!("{e:#}");
        std::process::exit(1);
    }
}

fn run(args: Args) -> anyhow::Result<()> {
    let extract = parse_extract(&args.extract)?;
    Ok(())
}

fn parse_extract(params: &ArgsExtract) -> anyhow::Result<ExtractedParam> {
    let extracted = match params {
        ArgsExtract {
            fields: Some(value),
            ..
        } => ExtractedParam::Fields(parse_range(value)?),
        ArgsExtract {
            bytes: Some(value), ..
        } => ExtractedParam::Bytes(parse_range(value)?),
        ArgsExtract {
            chars: Some(value), ..
        } => ExtractedParam::Chars(parse_range(value)?),
        _ => unreachable!(),
    };
    Ok(extracted)
}
