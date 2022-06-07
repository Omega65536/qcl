use crate::{lexer::Lexer, parser::Parser};

mod lexer;
mod parser;

fn main() {
    let source = "123.123 999".to_string();

    let tokens = Lexer::new(source).lex();
    println!("{:?}", tokens);
    let ast = Parser::new(tokens).parse();
    println!("{:?}", ast);
}
