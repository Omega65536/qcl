use std::fmt;

#[derive(Debug)]
pub struct SyntaxError {
    message: String,
}

impl SyntaxError {
    pub fn new(message: String) -> SyntaxError {
        SyntaxError { message }
    }
}

impl fmt::Display for SyntaxError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "ParserError: {}", self.message)
    }
}
