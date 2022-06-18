use std::fmt;

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

impl fmt::Display for Token {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let string = match &self {
            Token::End => "the end".to_string(),
            Token::Newline => "a newline".to_string(),
            Token::Print => "the print keyword".to_string(),
            Token::Number(number) => format!("the number {}", number),
            Token::Identifier(name) => format!("the identifier {}", name),
            Token::Plus => "+".to_string(),
            Token::Minus => "-".to_string(),
            Token::Star => "*".to_string(),
            Token::Slash => "/".to_string(),
            Token::LeftParen => "(".to_string(),
            Token::RightParen => ")".to_string(),
            Token::LeftCurly => "{".to_string(),
            Token::RightCurly => "}".to_string(),
        };
        write!(f, "{}", string)
    }
}
