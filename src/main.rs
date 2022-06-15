use crate::interpreter::Interpreter;
use crate::lexer::Lexer;
use crate::parser::Parser;
use crate::qcl_error::QclError;

mod ast;
mod interpreter;
mod lexer;
mod parser;
mod qcl_error;
mod span;
mod token;

fn main() {
    let source = "(1 + 2) * 3".to_string();

    match interpret(source) {
        Ok(()) => println!("Success!"),
        Err(error) => println!("{}", error),
    }
}

fn interpret(source: String) -> Result<(), QclError> {
    let lexer_result = Lexer::new(source.clone()).lex();
    let tokens = match lexer_result {
        Ok(tokens) => tokens,
        Err(error) => return Err(error),
    };
    println!("{:?}", tokens);

    let parser_result = Parser::new(source.clone(), tokens).parse();
    let ast = match parser_result {
        Ok(ast) => ast,
        Err(error) => return Err(error),
    };
    println!("{:?}", ast);

    let interpreter_result = Interpreter::new(ast).interpret();
    let value = match interpreter_result {
        Ok(value) => value,
        Err(error) => return Err(error),
    };
    println!("{}", value);
    Ok(())
}
