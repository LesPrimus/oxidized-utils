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

fn parse_delimiter(raw: &str) -> anyhow::Result<u8> {
    match raw.as_bytes() {
        [d] => Ok(*d),
        _ => anyhow::bail!(r#"--delim "{raw}" must be a single byte"#),
    }
}

fn main() {
    if let Err(e) = run(Args::parse()) {
        eprintln!("{e:#}");
        std::process::exit(1);
    }
}

fn run(args: Args) -> anyhow::Result<()> {
    let extract = parse_extract(&args.extract)?;
    let delimiter = parse_delimiter(&args.delimiter)?;

    for file_name in &args.files {
        let reader = match NamedReader::open(file_name) {
            Ok(reader) => reader,
            Err(e) => {
                eprintln!("{file_name}: {e}");
                continue;
            }
        };

        match &extract {
            ExtractedParam::Fields(list) => get_fields(reader, list, delimiter)?,
            ExtractedParam::Bytes(list) => get_bytes(reader, list)?,
            ExtractedParam::Chars(list) => get_chars(reader, list)?,
        }
    }

    Ok(())
}

fn extract<T: Copy>(items: &[T], ranges: &[Range<usize>]) -> Vec<T> {
    ranges
        .iter()
        .cloned()
        .flat_map(|range| range.filter_map(|idx| items.get(idx)))
        .copied()
        .collect()
}

fn get_fields(reader: NamedReader, ranges: &[Range<usize>], delimiter: u8) -> anyhow::Result<()> {
    let mut rdr = csv::ReaderBuilder::new()
        .has_headers(false)
        .delimiter(delimiter)
        .from_reader(reader);

    let mut wtr = csv::WriterBuilder::new()
        .delimiter(delimiter)
        .from_writer(std::io::stdout());

    for record in rdr.records() {
        let record = record?;
        let fields: Vec<&str> = record.iter().collect();
        let out = extract(&fields, ranges);
        wtr.write_record(&out)?;
    }
    wtr.flush()?;
    Ok(())
}

fn get_bytes(reader: NamedReader, ranges: &[Range<usize>]) -> anyhow::Result<()> {
    for line in reader.lines() {
        let line = line?;
        let bytes = line.as_bytes();
        let out = extract(bytes, ranges);
        println!("{}", String::from_utf8_lossy(&out));
    }
    Ok(())
}

fn get_chars(reader: NamedReader, ranges: &[Range<usize>]) -> anyhow::Result<()> {
    for line in reader.lines() {
        let line = line?;
        let chars: Vec<char> = line.chars().collect();
        let out: String = extract(&chars, ranges).iter().collect();
        println!("{out}");
    }
    Ok(())
}
