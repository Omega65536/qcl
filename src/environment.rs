use crate::ast::Expression;
use std::collections::HashMap;

pub struct Environment {
    values: HashMap<String, Expression>,
}

impl Environment {
    pub fn new() -> Environment {
        Environment {
            values: HashMap::new(),
        }
    }
}
