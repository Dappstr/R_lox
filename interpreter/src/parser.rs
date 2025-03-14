use crate::token::{Token, TokenType, Value};
use std::boxed::Box;
use std::cmp::PartialEq;
use crate::expression::Expr;
use crate::expression::Expr::{Binary, Unary};
use crate::token::TokenType::*;
use crate::statement::Stmt;

pub struct Parser {
    tokens: Vec<Token>,
    pos: usize,
}

impl Parser {
    pub fn new(tokens: Vec<Token>) -> Parser {
        Parser { tokens, pos: 0 }
    }

    pub fn parse(&mut self) -> Vec<Stmt> {
        let mut statements = Vec::new();

        while !self.is_at_end() {
            match self.declaration() {
                Ok(stmt) => {
                    statements.push(stmt)
                },
                Err(e) => {
                    eprintln!("Parsing error: {}", e);
                    self.synchronize();
                }
            }
            if self.check(TokenType::EOF) {
                break;
            }
        }
        statements
    }


    fn is_at_end(&self) -> bool {
        self.pos >= self.tokens.len()
    }

    fn peek(&self) -> &Token {
        if self.is_at_end() {
            return self.tokens.last().unwrap();
        }
        &self.tokens[self.pos]
    }

    fn previous(&self) -> &Token {
        if self.pos == 0 {
            panic!("Previous token is empty");
        }
        &self.tokens[self.pos - 1]
    }

    fn check(&self, token_type: TokenType) -> bool {
        if self.is_at_end() {
            return false;
        }
        self.peek().get_type() == token_type
    }

    fn advance(&mut self) -> &Token {
        if !self.is_at_end() {
            self.pos += 1;
        }
        self.previous()
    }

    fn consume(&mut self, expected: TokenType, message: &str) -> Result<&Token, String> {
        if self.check(expected) {
            return Ok(self.advance());
        }

        Err(format!("{} at line {}", message, self.peek().get_line()))
    }

    fn match_token_types(&mut self, types: &[TokenType]) -> bool {
        for &token_type in types {
            if self.check(token_type) {
                self.advance();
                return true;
            }
        }
        false
    }

    fn synchronize(&mut self) {
        while !self.is_at_end() {
            if self.previous().get_type() == SEMICOLON {
                return;
            }

            match self.peek().get_type() {
                CLASS | FUN | VAR |
                FOR | IF | WHILE |
                PRINT | RETURN => {
                    return;
                }
                _ => {
                    self.advance();
                }
            }
        }
    }

    fn print_statement(&mut self) -> Result<Stmt, String> {
        let value = self.expression()?;
        self.consume(TokenType::SEMICOLON, "Expect ';' after value.")?;
        Ok(Stmt::Print(*value))
    }

    fn expression_statement(&mut self) -> Result<Stmt, String> {
        let expr = self.expression()?;
        self.consume(TokenType::SEMICOLON, "Expect ';' after expression.")?;
        Ok(Stmt::Expr(*expr))
    }

    fn declaration(&mut self) -> Result<Stmt, String> {
        if self.match_token_types(&[TokenType::VAR]) {
            self.var_declaration()
        } else {
            self.statement()
        }
    }

    fn var_declaration(&mut self) -> Result<Stmt, String> {
        let name = self.consume(TokenType::IDENTIFIER, "Expect variable name.")?.clone();
        let mut initializer: Option<Expr> = None;
        if self.match_token_types(&[TokenType::EQUAL]) {
            initializer = Some(*self.expression()?);
        }
        self.consume(TokenType::SEMICOLON, "Expect ';' after value.")?;
        Ok(Stmt::Var(name, initializer))
    }

    fn statement(&mut self) -> Result<Stmt, String> {
        if self.match_token_types(&[TokenType::PRINT]) {
            self.print_statement()
        } else if self.match_token_types(&[TokenType::LEFT_BRACE]) {
            Ok(Stmt::Block(self.block()?))
        } else {
            self.expression_statement()
        }
    }

    fn block(&mut self) -> Result<Vec<Stmt>, String> {
        let mut statements = Vec::new();
        while !self.check(TokenType::RIGHT_BRACE) && !self.is_at_end() {
            if let Ok(statement) = self.declaration() {
                statements.push(statement);
            }
        }

        self.consume(TokenType::RIGHT_BRACE, "Expected '}' after block.")?;
        Ok(statements)
    }

    fn expression(&mut self) -> Result<Box<Expr>, String> {
        self.assignment()
    }

    fn assignment(&mut self) -> Result<Box<Expr>, String> {
        let expr = self.equality()?;

        if self.match_token_types(&[TokenType::EQUAL]) {
            let equals = self.previous().clone();
            let value = self.assignment()?;

            if let Expr::Variable(name) = *expr {
                 return Ok(Box::new(Expr::Assign{name, value}));
            }
            return Err(format!("Invalid assignment target at line {}", equals.get_line()));
        }
        Ok(expr)
    }

    fn equality(&mut self) -> Result<Box<Expr>, String> {
        let mut expr = self.comparison()?;

        while self.match_token_types(&[TokenType::BANG_EQUAL, TokenType::EQUAL_EQUAL]) {
            let operator = self.previous().clone();
            let right = self.comparison()?;
            expr = Box::new(Expr::Binary {
                left: expr,
                operator,
                right,
            });
        }
        Ok(expr)
    }

    fn comparison(&mut self) -> Result<Box<Expr>, String> {
        let mut expr = self.term()?;

        while self.match_token_types(&[GREATER, GREATER_EQUAL, LESS, LESS_EQUAL]) {
            let operator = self.previous().clone();
            let right = self.term()?;
            expr = Box::new(Binary {
                left: expr,
                operator,
                right,
            });
        }
        Ok(expr)
    }

    fn term(&mut self) -> Result<Box<Expr>, String> {
        let mut expr = self.factor()?;

        while self.match_token_types(&[MINUS, PLUS]) {
            let operator = self.previous().clone();
            let right = self.factor()?;
            expr = Box::new(Binary {
                left: expr,
                operator,
                right,
            });
        }
        Ok(expr)
    }

    fn factor(&mut self) -> Result<Box<Expr>, String> {
        let mut expr = self.unary()?;
        while self.match_token_types(&[SLASH, STAR]) {
            let operator = self.previous().clone();
            let right = self.unary()?;
            expr = Box::new(Binary {
                left: expr,
                operator,
                right,
            });
        }
        Ok(expr)
    }

    fn unary(&mut self) -> Result<Box<Expr>, String> {
        if self.match_token_types(&[BANG, MINUS]) {
            let operator = self.previous().clone();
            let right = self.unary()?;
            return Ok(Box::new(Unary {
                operator,
                right,
            }));
        }
        Ok(self.primary()?)
    }

    fn primary(&mut self) -> Result<Box<Expr>, String> {
        if self.match_token_types(&[TokenType::FALSE]) {
            return Ok(Box::new(Expr::Literal(Value::Boolean(false))));
        }
        if self.match_token_types(&[TokenType::TRUE]) {
            return Ok(Box::new(Expr::Literal(Value::Boolean(true))));
        }
        if self.match_token_types(&[TokenType::NIL]) {
            return Ok(Box::new(Expr::Literal(Value::Nil)));
        }
        if self.match_token_types(&[TokenType::NUMBER]) {
            let token = self.previous().clone();
            if let Some(Value::Number(n)) = token.literal {
                return Ok(Box::new(Expr::Literal(Value::Number(n))));
            }
        }
        if self.match_token_types(&[TokenType::STRING]) {
            let token = self.previous().clone();
            if let Some(Value::String(s)) = token.literal {
                return Ok(Box::new(Expr::Literal(Value::String(s))));
            }
        }
        if self.match_token_types(&[TokenType::LEFT_PAREN]) {
            let expr = self.expression()?;
            self.consume(TokenType::RIGHT_PAREN, "Expect ')' after expression.")?;
            return Ok(Box::new(Expr::Grouping(expr)));
        }
        if self.match_token_types(&[TokenType::IDENTIFIER]) {
            return Ok(Box::new(Expr::Variable(self.previous().clone())));
        }
        Err("Expected expression.".to_string())
    }
}