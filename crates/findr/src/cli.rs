use clap::{Parser, ValueEnum};
use regex::Regex;

#[derive(Debug, Parser)]
pub struct Args {
    #[arg(value_name = "PATH", default_value = ".")]
    pub paths: Vec<String>,

    #[arg( short('n'), long("name"), value_name = "NAME", num_args = 0..)]
    pub names: Vec<Regex>,

    #[arg(short('t'), long("type"), value_name = "TYPE", value_enum, num_args = 0..)]
    pub entry_types: Vec<EntryType>,
}

#[derive(Debug, Clone, Eq, PartialEq, ValueEnum)]
pub enum EntryType {
    #[value(name = "d")]
    Dir,

    #[value(name = "f")]
    File,

    #[value(name = "l")]
    Link,
}
