use crate::ast::{Expression, Statement};
use crate::object::Object;
use crate::qcl_error::{QclError, QclErrorType};
use crate::span::Spanned;

pub struct Interpreter {
    ast: Spanned<Statement>,
}

impl Interpreter {
    pub fn new(ast: Spanned<Statement>) -> Self {
        Interpreter { ast }
    }

    pub fn interpret(&self) -> Result<(), QclError> {
        self.interpret_statement(&self.ast)
    }

    fn interpret_statement(&self, statement: &Spanned<Statement>) -> Result<(), QclError> {
        match &statement.item {
            Statement::Print(expression) => {
                let evaluated = self.interpret_expresssion(expression)?;
                println!("{:?}", evaluated);
                Ok(())
            }
            Statement::Expression(expression) => {
                self.interpret_expresssion(expression)?;
                Ok(())
            }
            Statement::Block(statements) => {
                for statement in statements {
                    self.interpret_statement(statement)?;
                }
                Ok(())
            }
        }
    }

    fn interpret_expresssion(&self, expression: &Spanned<Expression>) -> Result<Object, QclError> {
        match &expression.item {
            Expression::Number(number) => Ok(Object::Float(*number)),
            Expression::Name(string) => Ok(Object::Float(123.)),
            Expression::Negation(inner) => {
                let inner = self.interpret_expresssion(inner)?;
                match inner {
                    Object::Float(inner) => Ok(Object::Float(-inner)),
                }
            }
            Expression::Addition(left, right) => {
                let left = self.interpret_expresssion(left)?;
                let right = self.interpret_expresssion(right)?;
                match (left, right) {
                    (Object::Float(left), Object::Float(right)) => Ok(Object::Float(left + right)),
                }
            }
            Expression::Subtraction(left, right) => {
                let left = self.interpret_expresssion(left)?;
                let right = self.interpret_expresssion(right)?;
                match (left, right) {
                    (Object::Float(left), Object::Float(right)) => Ok(Object::Float(left - right)),
                }
            }
            Expression::Multiplication(left, right) => {
                let left = self.interpret_expresssion(left)?;
                let right = self.interpret_expresssion(right)?;
                match (left, right) {
                    (Object::Float(left), Object::Float(right)) => Ok(Object::Float(left * right)),
                }
            }
            Expression::Division(left, right) => {
                let left = self.interpret_expresssion(left)?;
                let right = self.interpret_expresssion(right)?;
                match (left, right) {
                    (_, Object::Float(right)) if right == 0.0 => Err(QclError::new(
                        QclErrorType::DivisionByZeroError,
                        expression.span.clone(),
                        ":(".to_string(),
                    )),
                    (Object::Float(left), Object::Float(right)) => Ok(Object::Float(left / right)),
                }
            }
        }
    }
}
