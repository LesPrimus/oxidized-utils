use std::{fmt::Display, ops::AddAssign};

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
pub struct FileInfo {
    pub lines: usize,
    pub words: usize,
    pub bytes: usize,
    pub chars: usize,
}

impl Display for FileInfo {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}\t{}\t{}\t{}",
            self.lines, self.words, self.bytes, self.chars
        )
    }
}

impl AddAssign for FileInfo {
    fn add_assign(&mut self, other: Self) {
        self.lines += other.lines;
        self.words += other.words;
        self.bytes += other.bytes;
        self.chars += other.chars;
    }
}
