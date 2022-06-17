use colored::Colorize;
use std::fmt;
use std::rc::Rc;

#[derive(Debug, Clone)]
pub struct Span {
    pub source: Rc<String>,
    pub start: usize,
    pub end: usize,
}

#[derive(Debug, Clone)]
pub struct Spanned<T> {
    pub item: T,
    pub span: Span,
}

impl Span {
    pub fn new(source: Rc<String>, start: usize, end: usize) -> Span {
        Span { source, start, end }
    }
}

impl fmt::Display for Span {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let line_start = self.source[..self.start].matches('\n').count();
        let line_end = self.source[..self.end].matches('\n').count();
        let line_start_index = if line_start > 0 {
            self.source
                .match_indices('\n')
                .nth(line_start - 1)
                .unwrap()
                .0
                + 1
        } else {
            0
        };
        let line = self.source.split('\n').nth(line_start).unwrap();
        let prefix = format!("{} | ", line_start);
        write!(
            f,
            "{}{}\n{}{}",
            prefix,
            line,
            " ".repeat(prefix.len() + self.start - line_start_index),
            "^".repeat(self.end - self.start + 1).bright_red()
        )
    }
}

impl<T> Spanned<T> {
    pub fn new(item: T, span: Span) -> Spanned<T> {
        Spanned { item, span }
    }
}
