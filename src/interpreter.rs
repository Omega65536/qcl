use crate::ast::Expression;
use crate::qcl_error::QclError;
use crate::span::Spanned;

pub struct Interpreter {
    ast: Spanned<Expression>,
}

impl Interpreter {
    pub fn new(ast: Spanned<Expression>) -> Self {
        Interpreter { ast }
    }

    pub fn interpret(&self) -> Result<f64, QclError> {
        self.expression(&self.ast)
    }

    fn expression(&self, spanned: &Spanned<Expression>) -> Result<f64, QclError> {
        match &spanned.item {
            Expression::Number(number) => Ok(*number),
            Expression::Negation(negated) => Ok(-self.expression(&*negated)?),
            Expression::Addition(left, right) => {
                Ok(self.expression(&*left)? + self.expression(&*right)?)
            }
            Expression::Subtraction(left, right) => {
                Ok(self.expression(&*left)? - self.expression(&*right)?)
            }
            Expression::Multiplication(left, right) => {
                Ok(self.expression(&*left)? * self.expression(&*right)?)
            }
            Expression::Division(left, right) => {
                let right_evaluated = self.expression(&*right)?;
                if right_evaluated == 0. {
                    return Err(QclError::DivisionByZeroError(":(".to_string()));
                }
                Ok(self.expression(&*left)? / right_evaluated)
            }
        }
    }
}
