#[derive(Debug, PartialEq, Clone)]
pub enum TokenType {
    INTEGER,
    PLUS,
    MINUS,
    MUL,
    DIV,
    LPAREN,
    RPAREN,
    EOF
}

#[derive(Debug, Clone)]
pub struct Token {
    pub token: TokenType,
    pub value: String
}

impl Token {
    pub fn new(token: TokenType, value: String) -> Token {
        Token {
            token: token,
            value: value
        }
    }
}
