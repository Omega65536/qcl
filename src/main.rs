use crate::{lexer::Lexer, parser::Parser};

mod span;
mod lexer;
mod parser;
mod parser_error;

fn main() {
    let source = "1 - 2 / 3".to_string();

    let tokens = Lexer::new(source.clone()).lex();
    println!("{:?}", tokens);
    let parser_result = Parser::new(source.clone(), tokens).parse();
    match parser_result {
        Ok(ast) => println!("{:?}", ast),
        Err(error) => println!("{}", error),
    }
}
