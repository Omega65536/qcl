use crate::qcl_error::QclError;
use crate::span::{Span, Spanned};
use crate::token::Token;
use std::rc::Rc;

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

    pub fn lex(&mut self) -> Result<Vec<Spanned<Token>>, QclError> {
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

    pub fn next_token(&mut self) -> Result<Spanned<Token>, QclError> {
        let token = match self.peek() {
            Some(ch) => {
                if *ch == ' ' || *ch == '\r' {
                    self.advance();
                    return self.next_token();
                }
                if ch.is_digit(10) {
                    return self.next_number();
                }
                if ch.is_alphabetic() || *ch == '_' {
                    return self.next_name();
                }
                let token = match ch {
                    '\n' => Spanned::new(
                        Token::Newline,
                        Span::new(self.source.clone(), self.index, self.index),
                    ),
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
                    '(' => Spanned::new(
                        Token::LeftParen,
                        Span::new(self.source.clone(), self.index, self.index),
                    ),
                    ')' => Spanned::new(
                        Token::RightParen,
                        Span::new(self.source.clone(), self.index, self.index),
                    ),
                    '{' => Spanned::new(
                        Token::LeftCurly,
                        Span::new(self.source.clone(), self.index, self.index),
                    ),
                    '}' => Spanned::new(
                        Token::RightCurly,
                        Span::new(self.source.clone(), self.index, self.index),
                    ),
                    ch => {
                        return Err(QclError::SyntaxError(
                            format!("Could not handle character: '{}'", ch),
                        ))
                    }
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

    pub fn next_number(&mut self) -> Result<Spanned<Token>, QclError> {
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

    pub fn next_name(&mut self) -> Result<Spanned<Token>, QclError> {
        let start_index = self.index;
        let mut string = String::new();
        while let Some(ch) = self.peek() {
            if ch.is_alphabetic() || *ch == '_' {
                string.push(*ch);
            } else {
                break;
            }
            self.advance();
        }
        let span = Span::new(self.source.clone(), start_index, self.index - 1);
        match string.as_str() {
            "print" => Ok(Spanned::new(Token::Print, span)),
            _ => Ok(Spanned::new(Token::Identifier(string), span)),
        }
    }

    pub fn advance(&mut self) {
        self.index += 1;
    }

    pub fn peek(&self) -> Option<&char> {
        self.chars.get(self.index)
    }
}
