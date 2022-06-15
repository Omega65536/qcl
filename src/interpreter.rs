use crate::ast::{Expression, Statement};
use crate::qcl_error::QclError;
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
                let evaluated = self.expression(expression)?;
                println!("{}", evaluated);
                Ok(())
            },
            Statement::Expression(expression) => {
                self.expression(expression)?;
                Ok(())
            },
            Statement::Block(statements) => {
                for statement in statements {
                    self.interpret_statement(statement)?;
                }
                Ok(())
            }
        }
    }

    fn expression(&self, expression: &Spanned<Expression>) -> Result<f64, QclError> {
        match &expression.item {
            Expression::Number(number) => Ok(*number),
            Expression::Negation(negated) => Ok(-self.expression(&*negated)?),
            Expression::Addition(left, right) => {
                Ok(self.expression(left)? + self.expression(right)?)
            }
            Expression::Subtraction(left, right) => {
                Ok(self.expression(left)? - self.expression(right)?)
            }
            Expression::Multiplication(left, right) => {
                Ok(self.expression(left)? * self.expression(right)?)
            }
            Expression::Division(left, right) => {
                let right_evaluated = self.expression(right)?;
                if right_evaluated == 0. {
                    return Err(QclError::DivisionByZeroError(":(".to_string()));
                }
                Ok(self.expression(&*left)? / right_evaluated)
            }
        }
    }
}
