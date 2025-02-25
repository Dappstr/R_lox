use crate::token::{Token, Value};

#[derive(Debug)]
pub enum Expr {
    Literal(Value),
    Unary { operator: Token, right: Box<Expr> },
    Binary { left: Box<Expr>, operator: Token, right: Box<Expr> },
    Grouping(Box<Expr>),
    Variable(Token),
}