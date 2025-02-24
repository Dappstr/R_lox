use std::collections::{HashMap};
use once_cell::sync::Lazy;
use crate::token;
use crate::token::{Token, TokenType};

static KEYWORDS: Lazy<HashMap<&'static str, TokenType>> = Lazy::new(|| {
    let mut m = HashMap::new();
    m.insert("and", TokenType::AND);
    m.insert("class", TokenType::CLASS);
    m.insert("else", TokenType::ELSE);
    m.insert("false", TokenType::FALSE);
    m.insert("fun", TokenType::FUN);
    m.insert("for", TokenType::FOR);
    m.insert("if", TokenType::IF);
    m.insert("nil", TokenType::NIL);
    m.insert("or", TokenType::OR);
    m.insert("print", TokenType::PRINT);
    m.insert("return", TokenType::RETURN);
    m.insert("super", TokenType::SUPER);
    m.insert("this", TokenType::THIS);
    m.insert("true", TokenType::TRUE);
    m.insert("var", TokenType::VAR);
    m.insert("while", TokenType::WHILE);
    m
});
pub struct Scanner {
    src: String,
    tokens: Vec<Token>,
    start: usize,
    current: usize,
    line: usize,
}

impl Scanner {
    pub fn new(src: String) -> Self {
        Self {
            src,
            tokens: Vec::new(),
            start: 0,
            current: 0,
            line: 1,
        }
    }

    fn is_at_end(&self) -> bool { self.current >= self.src.len() }
    fn advance(&mut self) -> char {
        let c = self.src.chars().nth(self.current).unwrap_or('\0');
        self.current += 1;
        c
    }

    fn match_char(&mut self, expected: char) -> bool {
        if self.is_at_end() || self.src.chars().nth(self.current).unwrap() != expected {
            return false;
        }
        self.current += 1;
        true
    }
    fn peek(&self) -> char {
        if self.is_at_end() {
            return '\0';
        }
        self.src.chars().nth(self.current).unwrap_or('\0')
    }


    fn peek_next(&self) -> char {
        if self.current + 1 >= self.src.len() {
            '\0'
        } else {
            self.src.chars().nth(self.current + 1).unwrap()
        }
    }
    fn add_null_token(&mut self, t: TokenType) { self.add_token(t, None) }
    fn add_token(&mut self, t: TokenType, v: Option<token::Value>) {
        let text = self.src[self.start..self.current].to_string();
        self.tokens.push(Token::new(t, text, v, self.line));
    }

    fn string(&mut self) -> Result<(), String> {
        while self.peek() != '\"' {
            if self.peek() == '\n' { self.line += 1;}
            self.advance();
        }
        if self.is_at_end() {
            return Err("Unterminated string.".to_string())
        }

        self.advance();
        let lit = token::Value::String(self.src[self.start + 1..self.current - 1].to_string());
        self.add_token(TokenType::STRING, Some(lit));
        Ok(())
    }

    fn identifier(&mut self) {
        while self.peek().is_alphanumeric() || self.peek() == '_' {
            self.advance();
        }
        let text = &self.src[self.start..self.current];
        let token_type = KEYWORDS.get(text).cloned().unwrap_or(TokenType::IDENTIFIER);
        self.add_token(token_type, None);
    }


    fn number(&mut self) {
        while self.peek().is_numeric() {
            self.advance();
        }
        if self.peek() == '.' && self.peek_next().is_numeric() {
            self.advance();
            while self.peek().is_numeric() {
                self.advance();
            }
        }
        let literal = token::Value::Number(self.src[self.start..self.current].parse::<f64>().unwrap());
        self.add_token(TokenType::NUMBER, Some(literal));
    }

    fn scan_token(&mut self) {
        let c: char = self.advance();
        match c {
            '(' => self.add_null_token(TokenType::LEFT_PAREN),
            ')' => self.add_null_token(TokenType::RIGHT_PAREN),
            '{' => self.add_null_token(TokenType::LEFT_BRACE),
            '}' => self.add_null_token(TokenType::RIGHT_BRACE),
            ',' => self.add_null_token(TokenType::COMMA),
            '.' => self.add_null_token(TokenType::DOT),
            '-' => self.add_null_token(TokenType::MINUS),
            '+' => self.add_null_token(TokenType::PLUS),
            ';' => self.add_null_token(TokenType::SEMICOLON),
            '/' => {
                if self.match_char('/') {
                    while !self.match_char('\n') {
                        if self.match_char('\n') {
                            self.line += 1;
                            break;
                        }
                    }
                } else if self.match_char('*') {
                    let mut depth = 1;
                    while depth > 0 {
                        if self.is_at_end() {
                            eprintln!("Unterminated block comment");
                            return;
                        }
                        if self.peek() == '\n' {
                            self.line += 1;
                        } else if self.peek() == '*' && self.peek_next() == '/' {
                            self.advance();
                            self.advance();
                            depth -= 1;
                        } else if self.peek() == '/' && self.peek_next() == '*' {
                            self.advance();
                            self.advance();
                            depth += 1;
                        }
                        self.advance();
                    }
                }
                else {
                    self.add_null_token(TokenType::SLASH);
                }
            },
            '*' => self.add_null_token(TokenType::STAR),
            '!' => {
                if self.match_char('=') {
                    self.add_null_token(TokenType::BANG_EQUAL)
                } else {
                    self.add_null_token(TokenType::BANG)
                }
            },
            '=' => {
                if self.match_char('=') {
                    self.add_null_token(TokenType::EQUAL_EQUAL)
                } else {
                    self.add_null_token(TokenType::EQUAL)
                }
            },
            '<' => {
                if self.match_char('=') {
                    self.add_null_token(TokenType::LESS_EQUAL)
                } else {
                    self.add_null_token(TokenType::LESS)
                }
            },
            '>' => {
                if self.match_char('=') {
                    self.add_null_token(TokenType::GREATER_EQUAL)
                } else {
                    self.add_null_token(TokenType::GREATER)
                }
            },
            ' ' | '\r' | '\t' => {},
            '\n' => self.line += 1,
            '\"' => {
                match self.string() {
                    Ok(_) => {},
                    Err(e) => eprintln!("{}", e)
                }
            },
            'a'..='z' | 'A'..='Z' | '_' => {
                self.start = self.current - 1;
                self.identifier();
            },
            '0'..='9' => {
                self.start = self.current - 1;
                self.number();
            }
            _ => { eprintln!("Unrecognized character: {}", c); return }
        }
    }

    pub fn scan_tokens(mut self) -> Vec<Token> {
        while !self.is_at_end() {
            self.start = self.current;
            self.scan_token();
        }
        self.tokens.push(Token::new(TokenType::EOF, "".to_string(), None, self.line));
        self.tokens
    }

}

