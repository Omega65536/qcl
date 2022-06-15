#[derive(Debug, PartialEq, Clone)]
pub enum Token {
    // Special tokens
    End,
    Newline,
    // Keywords
    Print,
    // Literals
    Number(String),
    Identifier(String),
    // Symbols
    Plus,
    Minus,
    Star,
    Slash,
    LeftParen,
    RightParen,
    LeftCurly,
    RightCurly,
}
