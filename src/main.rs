#[allow(dead_code)]
mod token;
mod lexer;
mod interpreter;

use self::token::Token;

fn main() {
    let t = Token::new(token::TokenType::DIV, "/".to_string());
    println!("{:?}", t);
}
