use crate::interpreter::Interpreter;
use crate::lexer::Lexer;
use crate::parser::Parser;
use crate::qcl_error::QclError;
use env_logger::Env;
use std::fs;

mod ast;
mod interpreter;
mod lexer;
mod parser;
mod qcl_error;
mod span;
mod token;

fn main() {
    env_logger::Builder::from_env(Env::default().default_filter_or("trace")).init();

    let source = fs::read_to_string("example.qcl").expect("Unable to read file!");
    println!("Source: \"\"\"{}\"\"\"", source);

    match interpret(source) {
        Ok(()) => (),
        Err(error) => println!("{}", error),
    }
}

fn interpret(source: String) -> Result<(), QclError> {
    println!("Lexing:");
    let lexer_result = Lexer::new(source.clone()).lex();
    let tokens = match lexer_result {
        Ok(tokens) => tokens,
        Err(error) => return Err(error),
    };
    println!("{:?}", tokens);

    println!("\nParsing:");
    let parser_result = Parser::new(source, tokens).parse();
    let ast = match parser_result {
        Ok(ast) => ast,
        Err(error) => return Err(error),
    };
    println!("{:?}", ast);

    println!("\nInterpreting:");
    let interpreter_result = Interpreter::new(ast).interpret();
    match interpreter_result {
        Ok(()) => (),
        Err(error) => return Err(error),
    };
    Ok(())
}
