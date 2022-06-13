use crate::lexer::Lexer;
use crate::parser::Parser;
use crate::syntax_error::SyntaxError;

mod lexer;
mod parser;
mod span;
mod syntax_error;

fn main() {
    let source = "1 - 2 / 3".to_string();

    match interpret(source) {
        Ok(()) => println!("Success!"),
        Err(error) => println!("{}", error),
    }
}

fn interpret(source: String) -> Result<(), SyntaxError> {
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
    Ok(())
}
