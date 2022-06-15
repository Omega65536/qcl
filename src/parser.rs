use crate::ast::Expression;
use crate::qcl_error::QclError;
use crate::span::{Span, Spanned};
use crate::token::Token;
use std::rc::Rc;

pub struct Parser {
    source: Rc<String>,
    tokens: Vec<Spanned<Token>>,
    index: usize,
}

impl Parser {
    pub fn new(source: String, tokens: Vec<Spanned<Token>>) -> Self {
        Parser {
            source: Rc::new(source),
            tokens,
            index: 0,
        }
    }

    pub fn parse(&mut self) -> Result<Spanned<Expression>, QclError> {
        self.parse_expression()
    }

    pub fn parse_expression(&mut self) -> Result<Spanned<Expression>, QclError> {
        self.parse_addition()
    }

    pub fn parse_addition(&mut self) -> Result<Spanned<Expression>, QclError> {
        let mut current = self.parse_multiplication()?;
        loop {
            match self.peek().item {
                Token::Plus => {
                    self.advance();
                    let right = self.parse_multiplication()?;
                    let span = Span::new(self.source.clone(), current.span.start, right.span.end);
                    current = Spanned::new(
                        Expression::Addition(Box::new(current), Box::new(right)),
                        span,
                    );
                }
                Token::Minus => {
                    self.advance();
                    let right = self.parse_multiplication()?;
                    let span = Span::new(self.source.clone(), current.span.start, right.span.end);
                    current = Spanned::new(
                        Expression::Subtraction(Box::new(current), Box::new(right)),
                        span,
                    );
                }
                _ => return Ok(current),
            }
        }
    }

    pub fn parse_multiplication(&mut self) -> Result<Spanned<Expression>, QclError> {
        let mut current = self.parse_unary()?;
        loop {
            let spanned = self.peek();
            match spanned.item {
                Token::Star => {
                    self.advance();
                    let right = self.parse_unary()?;
                    let span = Span::new(self.source.clone(), current.span.start, right.span.end);
                    current = Spanned::new(
                        Expression::Multiplication(Box::new(current), Box::new(right)),
                        span,
                    );
                }
                Token::Slash => {
                    self.advance();
                    let right = self.parse_unary()?;
                    let span = Span::new(self.source.clone(), current.span.start, right.span.end);
                    current = Spanned::new(
                        Expression::Division(Box::new(current), Box::new(right)),
                        span,
                    );
                }
                _ => return Ok(current),
            }
        }
    }

    pub fn parse_unary(&mut self) -> Result<Spanned<Expression>, QclError> {
        let current = self.peek();
        match current.item {
            Token::Number(string) => {
                self.advance();
                let number = string.parse().expect("Unable to parse number!");
                let span = Span::new(self.source.clone(), current.span.start, current.span.end);
                Ok(Spanned::new(Expression::Number(number), span))
            },
            Token::Minus => {
                self.advance();
                let next = Box::new(self.parse_unary()?);
                let span = Span::new(self.source.clone(), current.span.start, next.span.end);
                Ok(Spanned::new(Expression::Negation(next), span))
            },
            Token::LeftParen => {
                self.advance();
                let inner = self.parse_expression()?.item;
                let right_paren = match self.peek().item {
                    Token::RightParen => self.peek(), 
                    _ => return Err(QclError::SyntaxError("Expected a closing parenthesis".to_string()))
                };
                self.advance();
                let span = Span::new(self.source.clone(), current.span.start, right_paren.span.end);
                Ok(Spanned::new(inner, span))
            }
            _ => Err(QclError::SyntaxError("Failed to parse unary!".to_string())),
        }
    }

    pub fn peek(&self) -> Spanned<Token> {
        match self.index {
            i if i < self.tokens.len() => self.tokens[i].clone(),
            _ => panic!("Unexpected error! :("),
        }
    }

    pub fn advance(&mut self) {
        self.index += 1;
    }
}
