use crate::span::{Span, Spanned};
use crate::syntax_error::SyntaxError;
use std::rc::Rc;

#[derive(Debug, PartialEq, Clone)]
pub enum Token {
    Unknown(String),
    End,
    Number(String),
    Plus,
    Minus,
    Star,
    Slash,
}

pub struct Lexer {
    chars: Vec<char>,
    index: usize,
    source: Rc<String>,
}

impl Lexer {
    pub fn new(source: String) -> Self {
        Lexer {
            chars: source.chars().collect(),
            index: 0,
            source: Rc::new(source),
        }
    }

    pub fn lex(&mut self) -> Result<Vec<Spanned<Token>>, SyntaxError> {
        let mut tokens = Vec::new();
        loop {
            let next_token = self.next_token()?;
            match next_token.item {
                Token::End => {
                    tokens.push(next_token);
                    return Ok(tokens);
                }
                _ => tokens.push(next_token),
            }
        }
    }

    pub fn next_token(&mut self) -> Result<Spanned<Token>, SyntaxError> {
        let token = match self.peek() {
            Some(ch) => {
                if *ch == ' ' {
                    self.advance();
                    return self.next_token();
                }
                if ch.is_digit(10) {
                    return self.next_number();
                }
                let token = match ch {
                    '+' => Spanned::new(
                        Token::Plus,
                        Span::new(self.source.clone(), self.index, self.index),
                    ),
                    '-' => Spanned::new(
                        Token::Minus,
                        Span::new(self.source.clone(), self.index, self.index),
                    ),
                    '*' => Spanned::new(
                        Token::Star,
                        Span::new(self.source.clone(), self.index, self.index),
                    ),
                    '/' => Spanned::new(
                        Token::Slash,
                        Span::new(self.source.clone(), self.index, self.index),
                    ),
                    _ => Spanned::new(
                        Token::Unknown(ch.to_string()),
                        Span::new(self.source.clone(), self.index, self.index),
                    ),
                };
                self.advance();
                token
            }
            None => {
                return Ok(Spanned::new(
                    Token::End,
                    Span::new(self.source.clone(), self.index, self.index),
                ));
            }
        };
        Ok(token)
    }

    pub fn next_number(&mut self) -> Result<Spanned<Token>, SyntaxError> {
        let start_index = self.index;
        let mut string = String::new();
        while let Some(ch) = self.peek() {
            if ch.is_digit(10) {
                string.push(*ch);
            } else {
                break;
            }
            self.advance();
        }
        if let Some('.') = self.peek() {
            self.advance();
            string.push('.');
            while let Some(ch) = self.peek() {
                if ch.is_digit(10) {
                    string.push(*ch);
                } else {
                    break;
                }
                self.advance();
            }
        }
        Ok(Spanned::new(
            Token::Number(string),
            Span::new(self.source.clone(), start_index, self.index - 1),
        ))
    }

    pub fn advance(&mut self) {
        self.index += 1;
    }

    pub fn peek(&self) -> Option<&char> {
        self.chars.get(self.index)
    }
}
