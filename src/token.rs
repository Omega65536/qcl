#[derive(Debug, PartialEq, Clone)]
pub enum Token {
    End,
    Number(String),
    Plus,
    Minus,
    Star,
    Slash,
    LeftParen,
    RightParen,
}
