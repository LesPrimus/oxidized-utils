use std::{
    fs::File,
    io::{self, BufRead, BufReader, Read},
};

pub struct NamedReader {
    reader: Box<dyn BufRead>,
    name: String,
}

impl NamedReader {
    fn from_stdin() -> Self {
        Self {
            reader: Box::new(BufReader::new(io::stdin())),
            name: "stdin".to_string(),
        }
    }

    fn from_filename(filename: &str) -> io::Result<Self> {
        let file = File::open(filename)?;

        Ok(Self {
            reader: Box::new(BufReader::new(file)),
            name: filename.to_string(),
        })
    }

    pub fn open(filename: &str) -> io::Result<Self> {
        let buffer = match filename {
            "-" => Self::from_stdin(),
            _ => Self::from_filename(filename)?,
        };
        Ok(buffer)
    }

    pub fn print_header(&self, add_new_line: bool) {
        if add_new_line {
            println!();
        }
        println!("==> {} <==", self.name);
    }
}

impl Read for NamedReader {
    fn read(&mut self, buf: &mut [u8]) -> io::Result<usize> {
        self.reader.read(buf)
    }
}

impl BufRead for NamedReader {
    fn fill_buf(&mut self) -> io::Result<&[u8]> {
        self.reader.fill_buf()
    }

    fn consume(&mut self, amount: usize) {
        self.reader.consume(amount);
    }
}
