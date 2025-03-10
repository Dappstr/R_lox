#[allow(non_camel_case_types)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TokenType {
    // Single-character tokens.
    LEFT_PAREN, RIGHT_PAREN, LEFT_BRACE, RIGHT_BRACE,
    COMMA, DOT, MINUS, PLUS, SEMICOLON, SLASH, STAR,

    // One or two character tokens.
    BANG, BANG_EQUAL, EQUAL, EQUAL_EQUAL,
    GREATER, GREATER_EQUAL, LESS, LESS_EQUAL,

    // Literals.
    IDENTIFIER, STRING, NUMBER,

    // Keywords.
    AND, CLASS, ELSE, FALSE, FUN, FOR, IF, NIL, OR,
    PRINT, RETURN, SUPER, THIS, TRUE, VAR, WHILE,

    EOF
}

#[allow(dead_code)]
#[derive(Debug, Clone, PartialEq)]
pub enum Value {
    Number(f64),
    String(String),
    Boolean(bool),
    Nil
}

#[derive(Debug, Clone)]
pub struct Token {
    t: TokenType,
    lexeme: String,
    pub(crate) literal: Option<Value>,
    line: usize,
}
#[allow(dead_code)]
impl Token {
    pub fn new(t: TokenType, lexeme: String, literal: Option<Value>, line: usize) -> Token {
        Token { t, lexeme, literal, line }
    }
    pub fn get_type(&self) -> TokenType {
        self.t.clone()
    }
    pub fn get_lexeme(&self) -> &str {
        &self.lexeme
    }
    pub fn get_literal(self) -> Option<Value> {
        self.literal
    }
    pub fn get_line(&self) -> usize {
        self.line
    }
}
