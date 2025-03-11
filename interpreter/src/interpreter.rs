use crate::expression::Expr;
use crate::statement::Stmt;
use crate::token::{Value, Token, TokenType};

pub struct Interpreter;

impl Interpreter {
    pub fn new() -> Self {
        Self
    }

    pub fn interpret(&self, statements: Vec<Stmt>) {
        for stmt in statements {
            self.execute(&stmt);
        }
    }

    fn execute(&self, stmt: &Stmt) {
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

    fn evaluate(&self, expr: &Expr) -> Result<Value, String> {
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
            Expr::Variable(name) => { Err("Variables not yet supported!".to_string()) },
        }
    }
}