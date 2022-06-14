use std::fmt;

#[derive(Debug)]
pub enum QclError {
    SyntaxError(String),
    DivisionByZeroError(String),
}

impl fmt::Display for QclError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            QclError::SyntaxError(message) => write!(f, "SyntaxError: {}", message),
            QclError::DivisionByZeroError(message) => write!(f, "DivisionByZeroError: {}", message),
        }
    }
}
