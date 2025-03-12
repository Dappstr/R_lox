use crate::expression::Expr;
use crate::token::*;
#[derive(Debug)]
pub enum Stmt {
    Expr(Expr),
    Print(Expr),
    Var(Token, Option<Expr>),
}