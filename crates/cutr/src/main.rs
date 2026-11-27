use crate::cli::{Args, ArgsExtract};
use clap::Parser;
use common::NamedReader;
use common::parsers::{PositionList, parse_range};
use std::io::BufRead;
use std::ops::Range;

mod cli;

#[derive(Debug)]
pub enum ExtractedParam {
    Fields(PositionList),
    Bytes(PositionList),
    Chars(PositionList),
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

fn main() {
    if let Err(e) = run(Args::parse()) {
        eprintln!("{e:#}");
        std::process::exit(1);
    }
}

fn run(args: Args) -> anyhow::Result<()> {
    let extract = parse_extract(&args.extract)?;
    // todo add the delimiter check too!

    for file_name in &args.files {
        let reader = match NamedReader::open(file_name) {
            Ok(reader) => reader,
            Err(e) => {
                eprintln!("{e}");
                continue;
            }
        };

        match &extract {
            ExtractedParam::Fields(list) => get_fields(reader, list),
            ExtractedParam::Bytes(list) => get_bytes(reader, list),
            ExtractedParam::Chars(list) => get_chars(reader, list)?,
        }
    }

    Ok(())
}

fn get_fields(reader: NamedReader, position_list: &[Range<usize>]) {
    todo!()
}

fn get_bytes(reader: NamedReader, position_list: &[Range<usize>]) {
    todo!()
}

fn get_chars(reader: NamedReader, position_list: &[Range<usize>]) -> anyhow::Result<()> {
    for line in reader.lines() {
        let line = line?;
        let chars: Vec<char> = line.chars().collect();

        let mut out = String::new();

        for range in position_list {
            for idx in range.clone() {
                if let Some(&char) = chars.get(idx) {
                    out.push(char);
                }
            }
        }

        println!("{out}");
    }
    Ok(())
}
