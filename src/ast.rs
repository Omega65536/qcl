use crate::span::Spanned;

#[derive(Debug)]
pub enum Statement {
    Print(Box<Spanned<Expression>>),
    Expression(Box<Spanned<Expression>>),
    Block(Vec<Box<Spanned<Statement>>>),
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
