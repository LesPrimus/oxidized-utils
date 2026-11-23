use std::io::BufRead;

use clap::Parser;
use common::NamedReader;

use crate::{cli::Args, file_info::FileInfo};

mod cli;
mod file_info;

fn main() {
    if let Err(e) = run(Args::parse()) {
        eprintln!("{e:#}");
        std::process::exit(1);
    }
}

fn normalize_cli_args(mut args: Args) -> Args {
    if !(args.lines || args.words || args.bytes || args.chars) {
        args.lines = true;
        args.words = true;
        args.bytes = true;
    }
    args
}

fn run(args: Args) -> anyhow::Result<()> {
    let args = normalize_cli_args(args);

    let mut total = FileInfo::default();

    for filename in &args.files {
        match NamedReader::open(filename) {
            Ok(mut reader) => {
                match count(&mut reader) {
                    Ok(file_info) => {
                        show_result(&file_info, &args, reader.name());
                        total += file_info;
                    }
                    Err(e) => eprintln!("{filename}: {e}"),
                };
            }
            Err(e) => eprintln!("{filename}: {e}"),
        }
    }

    if args.files.len() > 1 {
        show_result(&total, &args, Some("total"));
    }
    Ok(())
}

fn count(mut reader: impl BufRead) -> std::io::Result<FileInfo> {
    let mut buf = vec![];
    let mut file_info = FileInfo::default();

    loop {
        buf.clear();
        let bytes = reader.read_until(b'\n', &mut buf)?;
        if bytes == 0 {
            break;
        }

        let text = String::from_utf8_lossy(&buf);
        file_info.words += text.split_whitespace().count();
        file_info.chars += text.chars().count();
        file_info.bytes += bytes;
        if buf.ends_with(b"\n") {
            file_info.lines += 1;
        }
    }
    Ok(file_info)
}

fn show_result(file_info: &FileInfo, args: &Args, file_name: Option<&str>) {
    let mut result = vec![];
    if args.lines {
        result.push(file_info.lines);
    }
    if args.words {
        result.push(file_info.words);
    }
    if args.bytes {
        result.push(file_info.bytes);
    }
    if args.chars {
        result.push(file_info.chars);
    }

    let result: String = result.into_iter().map(|n| format!("{n:>8}")).collect();

    match file_name {
        Some(value) => println!("{result} {value}"),
        None => println!("{result}"),
    }
}
