use std::rc::Rc;
use std::cell::RefCell;
use crate::environment::Environment;
use crate::expression::Expr;
use crate::statement::Stmt;
use crate::token::{Value, Token, TokenType};
pub struct Interpreter {
    environment: Rc<RefCell<Environment>>,
}

impl Interpreter {
    pub fn new() -> Self {
        Self {
            environment: Rc::new(RefCell::new(Environment::new())),
        }
    }

    pub fn interpret(&mut self, statements: Vec<Stmt>) {
        for stmt in statements {
            self.execute(&stmt);
        }
    }

    fn execute_block(&mut self, stmts: &[Stmt], new_env: Rc<RefCell<Environment>>) {
        let previous = self.environment.clone();

        for stmt in stmts {
            self.execute(stmt);
        }
        self.environment = previous;
    }

    fn execute(&mut self, stmt: &Stmt) {
        match stmt {
            Stmt::Expr(expr) => {
                let _ = self.evaluate(expr);
            }
            Stmt::Print(expr) => {
                let value = self.evaluate(expr);
                match value {
                    Ok(value) => {
                        println!("{}", self.stringify(value))
                    },
                    Err(error) => {
                        eprintln!("Runtime error: {}", error);
                    }
                }
            },
            Stmt::Var(name, value) => {
                let value = if let Some(expr) = value {
                    self.evaluate(expr).unwrap_or(Value::Nil)
                } else {
                    Value::Nil
                };
                self.environment.borrow_mut().define(name.get_lexeme().to_string(), value);
            },
            Stmt::Block(stmts) => {
                let new_env = Rc::new(RefCell::new(Environment::enclose(self.environment.clone())));
                self.execute_block(stmts, new_env);
            },
            Stmt::If {condition, then_branch, else_branch} => {
                match self.evaluate(condition) {
                    Ok(value) => {
                        if self.is_truthy(&value) {
                            self.execute(then_branch);
                        } else if let Some(else_stmt) = else_branch {
                            self.execute(else_stmt);
                        }
                    }
                    Err(error) => {
                        eprintln!("Runtime error in if statement: {}", error);
                    }
                }
            },
            Stmt::While {condition, body } => {
                while let Ok(value) = self.evaluate(condition) {
                    if !self.is_truthy(&value) {
                        break;
                    }
                    self.execute(body);
                }
            }
        }
    }

    fn stringify(&self, value: Value) -> String {
        match value {
            Value::Number(number) => number.to_string(),
            Value::Boolean(boolean) => boolean.to_string(),
            Value::String(string) => string,
            Value::Nil => "nil".to_string(),
        }
    }

    fn is_truthy(&self, value: &Value) -> bool {
        match value {
            Value::Boolean(b) => *b,
            Value::Nil => false,
            _ => true,
        }
    }

    fn evaluate(&mut self, expr: &Expr) -> Result<Value, String> {
        match expr {
            Expr::Literal(value) => Ok(value.clone()),
            Expr::Unary { operator, right} => {
                let right = self.evaluate(right)?;
                match operator.get_type() {
                    TokenType::MINUS => match right {
                        Value::Number(value) => Ok(Value::Number(-value)),
                        _ => Err(format!("Not a number: {:?}", operator)),
                    },
                    TokenType::BANG => Ok(Value::Boolean(!self.is_truthy(&right))),
                    _ => Err(format!("Unknown unary operator: {:?}", operator)),
                }
            }
            Expr::Binary { operator, left, right } => {
                let left = self.evaluate(&left)?;
                let right = self.evaluate(&right)?;

                match operator.get_type() {
                    TokenType::PLUS => match (left, right) {
                        (Value::Number(left), Value::Number(right)) => Ok(Value::Number(left + right)),
                        (Value::String(left), Value::String(right)) => Ok(Value::String(left + &right)),
                        _ => Err(format!("Error {:?} not supported or mismatching types.", operator)),
                    },
                    TokenType::MINUS => match (left, right) {
                        (Value::Number(left), Value::Number(right)) => Ok(Value::Number(left - right)),
                        _ => Err(format!("Not a number or non-numeric values for operator: {:?}", operator)),
                    },
                    TokenType::STAR => match (left, right) {
                        (Value::Number(left), Value::Number(right)) => Ok(Value::Number(left * right)),
                        _ => Err(format!("Error {:?} not supported or mismatching types.", operator)),
                    },
                    TokenType::SLASH => match (left, right) {
                        (Value::Number(left), Value::Number(right)) => {
                            if right == 0.0 {
                                Err("Division by zero not allowed.".to_string())
                            } else {
                                Ok(Value::Number(left / right))
                            }
                        }
                        _ => Err(format!("Error {:?} not supported or types not numeric.", operator)),
                    }
                    TokenType::EQUAL_EQUAL => Ok(Value::Boolean(left == right)),
                    TokenType::BANG_EQUAL => Ok(Value::Boolean(left != right)),
                    TokenType::GREATER => match (left, right) {
                        (Value::Number(left), Value::Number(right)) => Ok(Value::Boolean(left > right)),
                        _ => Err(format!("Error {:?} not supported or mismatching types.", operator)),
                    },
                    TokenType::GREATER_EQUAL => match (left, right) {
                        (Value::Number(left), Value::Number(right)) => Ok(Value::Boolean(left >= right)),
                        _ => Err(format!("Error {:?} not supported or mismatching types.", operator)),
                    },
                    TokenType::LESS => match (left, right) {
                        (Value::Number(left), Value::Number(right)) => Ok(Value::Boolean(left < right)),
                        _ => Err(format!("Error {:?} not supported or mismatching types.", operator)),
                    },
                    TokenType::LESS_EQUAL => match (left, right) {
                        (Value::Number(left), Value::Number(right)) => Ok(Value::Boolean(left <= right)),
                        _ => Err(format!("Error {:?} not supported or mismatching types.", operator)),
                    },
                    _ => Err(format!("Error {:?} unknown binary operator.", operator)),
                }
            },
            Expr::Grouping(expr) => self.evaluate(expr),
            Expr::Variable(name) => self.environment.borrow().get(name),
            Expr::Assign { name, value } => {
                let value = self.evaluate(value)?;
                self.environment.borrow_mut().assign(name, value.clone())?;
                Ok(value)
            }
        }
    }
}