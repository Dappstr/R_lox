use crate::expression::Expr;

#[derive(Debug)]
pub enum Stmt {
    Expr(Expr),
    Print(Expr),
}