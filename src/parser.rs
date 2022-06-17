use crate::ast::{Expression, Statement};
use crate::qcl_error::{QclError, QclErrorType};
use crate::span::{Span, Spanned};
use crate::token::Token;
use log::trace;
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

    pub fn parse(&mut self) -> Result<Spanned<Statement>, QclError> {
        trace!("Parsing");
        let statement = self.parse_statement()?;
        self.advance_specific(Token::End)?;
        Ok(statement)
    }

    fn parse_statement(&mut self) -> Result<Spanned<Statement>, QclError> {
        trace!("Parsing statement");
        let statement = match self.peek().item {
            Token::LeftCurly => self.parse_block(),
            Token::Print => self.parse_print(),
            _ => {
                let expression = self.parse_expression()?;
                let span = Span::new(
                    self.source.clone(),
                    expression.span.start,
                    expression.span.end,
                );
                Ok(Spanned::new(
                    Statement::Expression(Box::new(expression)),
                    span,
                ))
            }
        }?;
        self.advance_specific(Token::Newline)?;
        Ok(statement)
    }

    fn parse_block(&mut self) -> Result<Spanned<Statement>, QclError> {
        trace!("Parsing block");
        let left_curly = self.advance_specific(Token::LeftCurly)?;
        let mut statements = Vec::new();
        while self.peek().item != Token::RightCurly {
            if self.peek().item == Token::Newline {
                self.advance();
            } else {
                statements.push(self.parse_statement()?);
            }
        }
        let right_curly = self.advance_specific(Token::RightCurly)?;
        let span = Span::new(
            self.source.clone(),
            left_curly.span.start,
            right_curly.span.end,
        );
        Ok(Spanned::new(Statement::Block(statements), span))
    }

    fn parse_print(&mut self) -> Result<Spanned<Statement>, QclError> {
        trace!("Parsing print");
        let print = self.advance_specific(Token::Print)?;
        let inner = self.parse_expression()?;
        let span = Span::new(self.source.clone(), print.span.start, inner.span.end);
        Ok(Spanned::new(Statement::Print(Box::new(inner)), span))
    }

    fn parse_expression(&mut self) -> Result<Spanned<Expression>, QclError> {
        trace!("Parsing expression");
        self.parse_addition()
    }

    fn parse_addition(&mut self) -> Result<Spanned<Expression>, QclError> {
        trace!("Parsing addition");
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

    fn parse_multiplication(&mut self) -> Result<Spanned<Expression>, QclError> {
        trace!("Parsing multiplication");
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

    fn parse_unary(&mut self) -> Result<Spanned<Expression>, QclError> {
        trace!("Parsing unary");
        let current = self.peek();
        match current.item {
            Token::Number(string) => {
                self.advance();
                let number = string.parse().expect("Unable to parse number!");
                let span = Span::new(self.source.clone(), current.span.start, current.span.end);
                Ok(Spanned::new(Expression::Number(number), span))
            }
            Token::Minus => {
                self.advance();
                let next = Box::new(self.parse_unary()?);
                let span = Span::new(self.source.clone(), current.span.start, next.span.end);
                Ok(Spanned::new(Expression::Negation(next), span))
            }
            Token::LeftParen => {
                self.advance();
                let inner = self.parse_expression()?.item;
                let right_paren = self.advance_specific(Token::RightParen)?;
                let span = Span::new(
                    self.source.clone(),
                    current.span.start,
                    right_paren.span.end,
                );
                Ok(Spanned::new(inner, span))
            }
            _ => Err(QclError::new(
                QclErrorType::SyntaxError,
                current.span.clone(),
                format!("Unexpected token {:?}", current.item),
            )),
        }
    }

    fn peek(&self) -> Spanned<Token> {
        match self.index {
            i if i < self.tokens.len() => self.tokens[i].clone(),
            _ => panic!("Unexpected error :("),
        }
    }

    fn advance(&mut self) {
        self.index += 1;
    }

    fn advance_specific(&mut self, expected: Token) -> Result<Spanned<Token>, QclError> {
        let token = self.peek();
        if token.item == expected {
            self.advance();
            Ok(token)
        } else {
            Err(QclError::new(
                QclErrorType::SyntaxError,
                token.span.clone(),
                format!("Expected {:?} but found {:?}", expected, token.item),
            ))
        }
    }
}
