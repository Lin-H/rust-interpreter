use super::lexer::Lexer;
use super::token::Token;
use super::token::TokenType;
use super::parser::Parser;

pub struct Interpreter<'a> {
    parser: &'a mut Parser<'a>,
}

impl<'a> Interpreter<'a> {
    pub fn new(parser: &'a mut Parser<'a>) -> Interpreter<'a> {
        Interpreter {
            parser: parser
        }
    }
}