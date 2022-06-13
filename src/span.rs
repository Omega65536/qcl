use std::rc::Rc;

#[derive(Debug, Clone)]
pub struct Span {
    pub source: Rc<String>,
    pub start: usize,
    pub end: usize
}

#[derive(Debug, Clone)]
pub struct Spanned<T> {
    pub item: T,
    pub span: Span
}

impl Span {
    pub fn new(source: Rc<String>, start: usize, end: usize) -> Span {
        Span {
            source,
            start,
            end
        }
    }
}

impl<T> Spanned<T> {
    pub fn new(item: T, span: Span) -> Spanned<T> {
        Spanned {
            item,
            span
        }
    }
}