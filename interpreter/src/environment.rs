use std::collections::HashMap;
use crate::expression::Expr;
use crate::token::*;

pub struct Environment {
    values: HashMap<String, Value>,
    parent: Option<Box<Environment>>,
}

impl Environment {
    pub fn new() -> Environment {
        Self { values: HashMap::new(), parent: None, }
    }

    pub fn define(&mut self, name: String, value: Value) {
        self.values.insert(name, value);
    }

    pub fn enclose(parent: Environment) -> Environment {
        Self { values: HashMap::new(), parent: Some(Box::new(parent)) }
    }

    pub fn assign(&mut self, name: &Token, value: Value) -> Result<Value, String> {
        if self.values.contains_key(name.get_lexeme()) {
            self.values.insert(name.get_lexeme().to_string(), value.clone());
            return Ok(value);
        }
        Err(format!("Variable {} not defined", name.get_lexeme()))
    }

    pub fn get(&self, name: &Token) -> Result<Value, String> {
        match self.values.get(name.get_lexeme()) {
            Some(existing_value) => Ok(existing_value.clone()),
            None => Err(format!("Variable {} not defined", name.get_lexeme()))
        }
    }
}

impl Clone for Environment {
    fn clone(&self) -> Self {
        Self { values: self.values.clone(), parent: self.parent.clone() }
    }
}