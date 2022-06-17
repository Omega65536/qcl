use crate::span::Span;
use colored::Colorize;
use std::fmt::{self, Debug};

#[derive(Debug)]
pub enum QclErrorType {
    SyntaxError,
    DivisionByZeroError,
}

#[derive(Debug)]
pub struct QclError {
    error_type: QclErrorType,
    span: Span,
    message: String,
}

impl QclError {
    pub fn new(error_type: QclErrorType, span: Span, message: String) -> QclError {
        QclError {
            error_type,
            span,
            message,
        }
    }
}

impl fmt::Display for QclError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "{}\n{}",
            self.span,
            format!("{:?}: {}", self.error_type, self.message).bright_red()
        )
    }
}
