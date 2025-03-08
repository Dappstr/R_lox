use crate::token::{Token, TokenType, Value};
use std::boxed::Box;
use std::cmp::PartialEq;
use crate::expression::Expr;
use crate::expression::Expr::{Binary, Unary};
use crate::token::TokenType::*;

pub struct Parser {
    tokens: Vec<Token>,
    pos: usize,
}

impl Parser {
    pub fn new(tokens: Vec<Token>) -> Parser {
        Parser { tokens, pos: 0 }
    }

    pub fn parse(&mut self) -> Vec<Expr> {
        let mut expressions = Vec::new();

        while !self.is_at_end() {
            let expr_result = self.expression();

            match expr_result {
                Ok(expr) => expressions.push(*expr),
                Err(msg) => {
                    eprintln!("Parse error: {}", msg);
                    self.synchronize();
                    break;
                }
            }

            if self.check(TokenType::EOF) {
                break;
            }
        }
        expressions
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

    fn expression(&mut self) -> Result<Box<Expr>, String> {
        self.equality()
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

        Err("Expected expression.".to_string())
    }
}