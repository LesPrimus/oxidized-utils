use crate::cli::{Args, EntryType};
use clap::Parser;
use regex::Regex;
use walkdir::{WalkDir, DirEntry};

mod cli;

fn main() {
    if let Err(e) = run(Args::parse()) {
        eprintln!("{e}");
        std::process::exit(1);
    }
}

fn run(args: Args) -> anyhow::Result<()> {
    for path in args.paths.iter() {
        filter_path(path, &args);
    }
    Ok(())
}

fn filter_names(entry: &DirEntry, names: &[Regex]) -> bool {
    let name = entry.file_name().to_string_lossy();
    names.is_empty() || names.iter().any(|r| r.is_match(&name))
}

fn filter_entry_types(entry: &DirEntry, types: &[EntryType]) -> bool {
    types.is_empty() || types.iter().any(|entry_type| match entry_type {
        EntryType::File => entry.file_type().is_file(),
        EntryType::Dir => entry.file_type().is_dir(),
        EntryType::Link => entry.file_type().is_symlink(),
    })
}

fn filter_path(path: &str, args: &Args) {
    let entries = WalkDir::new(path)
        .into_iter()
        .filter_map(|entry_result| match entry_result {
            Ok(entry) => Some(entry),
            Err(e) => {
                eprintln!("{e}");
                None
            }
        })
        .filter(|entry| filter_names(entry, &args.names))
        .filter(|entry| filter_entry_types(entry, &args.entry_types))
        .map(|dir_entry| dir_entry.path().display().to_string())
        .collect::<Vec<_>>();

    println!("{}", entries.join("\n"));
}
