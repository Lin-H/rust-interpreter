
mod token;

use self::token::Token;

fn main() {
    let t = Token::new(token::TokenType::DIV, "/".to_string());
    println!("{:?}", t);
}
