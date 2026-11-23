#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
pub struct FileInfo {
    pub lines: usize,
    pub words: usize,
    pub bytes: usize,
    pub chars: usize,
}
