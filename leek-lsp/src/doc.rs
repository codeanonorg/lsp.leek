
use tower_lsp::lsp_types::*;
use std::ops::{Deref, Index, RangeBounds};
use tower_lsp::lsp_types::Range as LSPRange;
use std::slice::SliceIndex;

#[derive(Clone, Debug, Default, Eq, PartialEq)]
pub struct TextDocument {
    data: String,
    lines: Vec<(usize, usize)>,
}

impl From<String> for TextDocument {
    fn from(data: String) -> Self {
        let lines = Self::make_line_offsets(&data);
        Self { data, lines }
    }
}

impl AsRef<str> for TextDocument {
    fn as_ref(&self) -> &str {
        &self.data
    }
}

impl Deref for TextDocument {
    type Target = str;

    fn deref(&self) -> &Self::Target {
        &self.data
    }
}

impl<R> Index<R> for TextDocument where String: Index<R> {
    type Output = <String as Index<R>>::Output;

    fn index(&self, index: R) -> &Self::Output {
        &self.data[index]
    }
}

impl TextDocument {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn get_lsprange(&self, range: LSPRange) -> Option<&str> {
        self.get(self.lookup_offset(range.start)?..self.lookup_offset(range.end)?)
    }

    pub fn get<R: SliceIndex<str>>(&self, range: R) -> Option<&R::Output> {
        self.data.get(range)
    }

    pub fn lookup_offset(&self, pos: Position) -> Option<usize> {
        self.lines
            .get(pos.line as usize)
            .map(|(off, _)| off + pos.character as usize)
    }

    pub fn lookup_pos(&self, offset: usize) -> Option<Position> {
        eprintln!("[lookup_pos] offset={}, lines={:?}", offset, self.lines);
        let (line, (off, _)) = self
            .lines
            .iter()
            .cloned()
            .enumerate()
            .skip_while(|(_, (off, len))| off + len < offset)
            .next()?;
        eprintln!("[lookup_pos] line={}, off={}", line, off);
        Some(Position {
            line: line as u64,
            character: (offset - off) as u64,
        })
    }

    pub fn update(&mut self, mut change: TextDocumentContentChangeEvent) -> Option<()> {
        if let Some(range) = change.range {
            let start = self.lookup_offset(range.start)?;
            let end = self.lookup_offset(range.end)?;
            self.data = format!("{}{}{}", &self.data[..start], change.text, &self.data[end+1..]);
            self.lines = Self::make_line_offsets(&self.data);
        } else {
            *self = Self::from(change.text);
        }
        eprintln!("[update] view={:?}", self.data);
        Some(())
    }

    fn make_line_offsets(data: &str) -> Vec<(usize, usize)> {
        let mut len = 0;
        let mut off = 0;
        data.chars()
            .enumerate()
            .filter_map(|(i, c)| {
                if c == '\n' {
                    let linelen = len;
                    let o = off;
                    off = i;
                    len = 0;
                    Some((o, linelen))
                } else {
                    len += 1;
                    None
                }
            })
            .collect()
    }
}