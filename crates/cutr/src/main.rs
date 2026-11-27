use std::ops::Range;
use clap::Parser;

use crate::cli::{Args, ArgsExtract};

mod cli;

type PositionList = Vec<Range<usize>>;
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

fn parse_extract(params: &ArgsExtract) -> anyhow::Result<ExtractedParam>{
    // todo extract ranges here..
    let extracted = match params {
        ArgsExtract { fields: Some(value), ..} => ExtractedParam::Fields(todo!()),
        ArgsExtract { bytes: Some(value), ..} => ExtractedParam::Bytes(todo!()),
        ArgsExtract { chars: Some(value), ..} => ExtractedParam::Chars(todo!()),
        _ => unreachable!()
    };
    Ok(extracted)
}
