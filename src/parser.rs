use std::rc::Rc;
use crate::span::{Span, Spanned};
use crate::lexer::Token;
use crate::parser_error::ParserError;

#[derive(Debug)]
pub enum Expression {
    Addition(Box<Spanned<Expression>>, Box<Spanned<Expression>>),
    Subtraction(Box<Spanned<Expression>>, Box<Spanned<Expression>>),
    Multiplication(Box<Spanned<Expression>>, Box<Spanned<Expression>>),
    Division(Box<Spanned<Expression>>, Box<Spanned<Expression>>),
    Negation(Box<Spanned<Expression>>),
    Number(f64),
}

pub struct Parser {
    source: Rc<String>,
    tokens: Vec<Spanned<Token>>,
    index: usize,
}

impl Parser {
    pub fn new(source: String, tokens: Vec<Spanned<Token>>) -> Self {
        Parser { source: Rc::new(source), tokens, index: 0 }
    }

    pub fn parse(&mut self) -> Result<Spanned<Expression>, ParserError> {
        self.parse_expression()
    }

    pub fn parse_expression(&mut self) -> Result<Spanned<Expression>, ParserError> {
        self.parse_addition()
    }

    pub fn parse_addition(&mut self) -> Result<Spanned<Expression>, ParserError> {
        let mut current = self.parse_multiplication()?;
        loop {
            match self.peek().item {
                Token::Plus => {
                    self.consume();
                    let right = self.parse_multiplication()?;
                    let span = Span::new(self.source.clone(), current.span.start, right.span.end);
                    current = Spanned::new(Expression::Addition(
                        Box::new(current),
                        Box::new(right),
                    ), span);
                }
                Token::Minus => {
                    self.consume();
                    let right = self.parse_multiplication()?;
                    let span = Span::new(self.source.clone(), current.span.start, right.span.end);
                    current = Spanned::new(Expression::Subtraction(
                        Box::new(current),
                        Box::new(right),
                    ), span);
                }
                _ => return Ok(current),
            }
        }
    }

    pub fn parse_multiplication(&mut self) -> Result<Spanned<Expression>, ParserError> {
        let mut current = self.parse_unary()?;
        loop {
            let spanned = self.peek();
            match spanned.item {
                Token::Star => {
                    self.consume();
                    let right = self.parse_unary()?;
                    let span =  Span::new(self.source.clone(), current.span.start, right.span.end);
                    current = Spanned::new(Expression::Multiplication(Box::new(current), Box::new(right)), span);
                }
                Token::Slash => {
                    self.consume();
                    let right = self.parse_unary()?;
                    let span = Span::new(self.source.clone(), current.span.start, right.span.end);
                    current = Spanned::new(Expression::Division(Box::new(current), Box::new(right)), span);
                }
                _ => return Ok(current),
            }
        }
    }

    pub fn parse_unary(&mut self) -> Result<Spanned<Expression>, ParserError> {
        let current = self.peek();
        match current.item {
            Token::Number(string) => {
                self.consume();
                let number = string.parse().expect("Unable to parse number!");
                let span = Span::new(self.source.clone(), current.span.start, current.span.end);
                Ok(Spanned::new(Expression::Number(number), span))
            },
            Token::Minus => {
                self.consume();
                let next = Box::new(self.parse_unary()?);
                let span = Span::new(self.source.clone(), current.span.start, next.span.end);
                Ok(Spanned::new(Expression::Negation(next), span))
            }
            _ =>  Err(ParserError::new("Failed to parse unary!".to_string())),
        }
    }

    // pub fn check(&mut self, expected_token_type: TokenType) -> &Token {
    //     let current_token = self.tokens.get(self.index);
    // }

    pub fn peek(&self) -> Spanned<Token> {
        match self.index {
            i if i < self.tokens.len() => self.tokens[i].clone(),
            _ => panic!("Unexpected error! :("),
        }
    }

    pub fn consume(&mut self) -> &Spanned<Token> {
        let next_token = self.tokens.get(self.index);
        self.index += 1;
        next_token.unwrap()
    }

    // pub fn consume_specific(&mut self, expected_token_type: TokenType) -> &Token {
    //     let next_token = self.tokens.get(self.index).unwrap();
    //     self.index += 1;
    //     if next_token.token_type == expected_token_type {
    //         next_token
    //     } else {
    //         panic!(
    //             "Expected a token of type {:?}, but found {:?}",
    //             expected_token_type, next_token
    //         );
    //     }
    // }
}
