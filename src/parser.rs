use crate::lexer::Token;

#[derive(Debug)]
pub enum AST {
    Unknown
}

pub struct Parser {
    tokens: Vec<Token>,
    index: usize
}

impl Parser {
    pub fn new(tokens: Vec<Token>) -> Self {
        Parser {
            tokens: tokens,
            index: 0
        }
    }

    pub fn parse(&self) -> AST {
        AST::Unknown
    }
}