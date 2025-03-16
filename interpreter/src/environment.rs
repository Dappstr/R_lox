use std::collections::HashMap;
use std::rc::Rc;
use std::cell::RefCell;
use crate::expression::Expr;
use crate::token::*;

#[derive(Debug, Clone)]
pub struct Environment {
    values: HashMap<String, Value>,
    parent: Option<Rc<RefCell<Environment>>>,
}

impl Environment {
    pub fn new() -> Self {
        Self { values: HashMap::new(), parent: None }
    }

    pub fn enclose(parent: Rc<RefCell<Environment>>) -> Self {
        Self { values: HashMap::new(), parent: Some(parent) }
    }

    pub fn define(&mut self, name: String, value: Value) {
        self.values.insert(name, value);
    }

    pub fn assign(&mut self, name: &Token, value: Value) -> Result<Value, String> {
        if self.values.contains_key(name.get_lexeme()) {
            self.values.insert(name.get_lexeme().to_string(), value.clone());
            return Ok(value);
        }
        if let Some(parent) = &self.parent {
            return parent.borrow_mut().assign(name, value);
        }
        Err(format!("Runtime error: Variable {} not defined", name.get_lexeme()))
    }

    pub fn get(&self, name: &Token) -> Result<Value, String> {
        match self.values.get(name.get_lexeme()) {
            Some(existing_value) => Ok(existing_value.clone()),
            None => {
                if let Some(parent) = &self.parent {
                    return parent.borrow().get(name);
                }
                Err(format!("Runtime error: Variable {} not defined", name.get_lexeme()))
            }
        }
    }
}
