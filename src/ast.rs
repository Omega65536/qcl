use crate::span::Spanned;

#[derive(Debug)]
pub enum Statement {
    Block(Vec<Spanned<Statement>>),
    Print(Box<Spanned<Expression>>),
    Expression(Box<Spanned<Expression>>),
}

#[derive(Debug)]
pub enum Expression {
    Number(f64),
    Negation(Box<Spanned<Expression>>),
    Addition(Box<Spanned<Expression>>, Box<Spanned<Expression>>),
    Subtraction(Box<Spanned<Expression>>, Box<Spanned<Expression>>),
    Multiplication(Box<Spanned<Expression>>, Box<Spanned<Expression>>),
    Division(Box<Spanned<Expression>>, Box<Spanned<Expression>>),
}
