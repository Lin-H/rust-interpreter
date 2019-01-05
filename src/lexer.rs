use std::str::Chars;
use super::token::Token;
use super::token::TokenType;

pub struct Lexer<'a> {
    // client string input
    text: Chars<'a>,
    pos: usize,
    current_char: Option<char>
}

impl<'a> Lexer<'a> {
    pub fn new(text: &'a str) -> Lexer<'a> {
        let mut chars: Chars = text.chars();
        Lexer {
            current_char: chars.nth(0),
            text: chars,
            pos: 0
        }
    }
    // Advance the `pos` pointer and set the `current_char` variable.
    pub fn advance(&mut self) {
        self.pos += 1;
        let current_char = self.text.nth(self.pos);
        self.current_char = current_char;
    }
    pub fn skip_whitespace(&mut self) {
        while self.current_char.unwrap() == ' ' {
            self.advance()
        }
    }
    // Return a (multidigit) integer consumed from the input.
    pub fn integer(&mut self) -> String {
        let mut result: String = String::new();
        let char = self.current_char.unwrap();
        while char.is_digit(10) {
            result.push(char);
            self.advance();
        }
        result
    }
    /* Lexical analyzer (also known as scanner or tokenizer)
        This method is responsible for breaking a sentence
        apart into tokens. One token at a time.
    */
    pub fn get_next_token(&mut self) -> Token {
        while let Some(c) = self.current_char {
            if c == ' ' {
                self.skip_whitespace();
                continue;
            }
            if c.is_digit(10) {
                return Token::new(TokenType::INTEGER, self.integer());
            }
            if c == '+' {
                self.advance();
                return Token::new(TokenType::PLUS, String::from("+"));
            }
            if c == '-' {
                self.advance();
                return Token::new(TokenType::MINUS, String::from("-"));
            }
            if c == '*' {
                self.advance();
                return Token::new(TokenType::MUL, String::from("*"));
            }
            if c == '/' {
                self.advance();
                return Token::new(TokenType::DIV, String::from("/"));
            }
            if c == '(' {
                self.advance();
                return Token::new(TokenType::LPAREN, String::from("("));
            }
            if c == ')' {
                self.advance();
                return Token::new(TokenType::RPAREN, String::from(")"));
            }
            panic!("unknow symbol");
        }
        Token::new(TokenType::EOF, "".to_string())
    }
}
