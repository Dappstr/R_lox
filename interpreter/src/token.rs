#[allow(non_camel_case_types)]
#[derive(Debug, Clone)]
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
#[derive(Debug)]
pub enum Value {
    Number(f64),
    String(String),
    Boolean(bool),
}

#[derive(Debug)]
pub struct Token {
    t: TokenType,
    lexeme: String,
    literal: Option<Value>,
    line: usize,
}
#[allow(dead_code)]
impl Token {
    pub fn new(t: TokenType, lexeme: String, literal: Option<Value>, line: usize) -> Token {
        Token { t, lexeme, literal, line }
    }
    pub fn get_type(self) -> TokenType {
        self.t
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

// impl Display for Token {
//     fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
//         write!(f, "{:?}, {}, {:?}, {}", self.t, self.lexeme, self.literal, self.line)
//     }
// }