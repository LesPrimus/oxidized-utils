use std::{
    fs::File,
    io::{BufRead, BufWriter, Write},
};

use anyhow::Context;
use clap::Parser;
use common::NamedReader;

use crate::cli::Args;

mod cli;

fn main() {
    if let Err(e) = run(Args::parse()) {
        eprintln!("{e:#}");
        std::process::exit(1);
    }
}

fn run(args: Args) -> anyhow::Result<()> {
    let reader = NamedReader::open(&args.in_file).with_context(|| args.in_file.clone())?;
    let mut output = get_output(args.out_file.as_deref())?;

    print_output(reader, &mut output, args.count)?;

    Ok(())
}

fn get_output(filename: Option<&str>) -> anyhow::Result<Box<dyn Write>> {
    let out: Box<dyn Write> = match filename {
        Some(filename) => {
            let file = File::create(filename).with_context(|| filename.to_string())?;
            Box::new(BufWriter::new(file))
        }
        None => Box::new(BufWriter::new(std::io::stdout())),
    };
    Ok(out)
}

fn print_output(
    mut reader: impl BufRead,
    writer: &mut impl Write,
    add_count: bool,
) -> std::io::Result<()> {
    let mut buf = String::new();
    let mut previous = String::new();
    let mut count: usize = 0;

    let mut write_group = |count: usize, line: &str| -> std::io::Result<()> {
        if count > 0 {
            if add_count {
                write!(writer, "{count:>4} {line}")?;
            } else {
                write!(writer, "{line}")?;
            }
        }
        Ok(())
    };

    loop {
        buf.clear();
        let bytes = reader.read_line(&mut buf)?;
        if bytes == 0 {
            break;
        }

        if count > 0 && previous.trim_end() == buf.trim_end() {
            count += 1;
        } else {
            write_group(count, &previous)?;
            std::mem::swap(&mut previous, &mut buf);
            count = 1;
        }
    }

    write_group(count, &previous)?;

    writer.flush()?;
    Ok(())
}
