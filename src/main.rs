mod interpreter;
mod lexer;
#[allow(dead_code)]
mod token;
mod parser;

use std::io::{self, Write};

fn main() {
    let mut input = String::new();
    loop {
        print!("calc>");
        io::stdout().flush().unwrap();
        io::stdin().read_line(&mut input).unwrap();
        let mut lex = lexer::Lexer::new(&input);
        let mut my_interpreter = interpreter::Interpreter::new(&mut lex);
        println!("{:?}", my_interpreter.expr());
        input.clear();
    }
}

#[test]
fn add() {
    let mut lex = lexer::Lexer::new("1+1\r\n");
    let mut my_interpreter = interpreter::Interpreter::new(&mut lex);
    assert_eq!(2, my_interpreter.expr());
}
#[test]
fn minus() {
    let mut lex = lexer::Lexer::new("10-1\r\n");
    let mut my_interpreter = interpreter::Interpreter::new(&mut lex);
    assert_eq!(9, my_interpreter.expr());
}
#[test]
fn mul() {
    let mut lex = lexer::Lexer::new("7*3\r\n");
    let mut my_interpreter = interpreter::Interpreter::new(&mut lex);
    assert_eq!(21, my_interpreter.expr());
}

#[test]
fn div() {
    let mut lex = lexer::Lexer::new("8/2\r\n");
    let mut my_interpreter = interpreter::Interpreter::new(&mut lex);
    assert_eq!(4, my_interpreter.expr());
}

#[test]
fn paren() {
    let mut lex = lexer::Lexer::new("10 * (2+4) / 4\r\n");
    let mut my_interpreter = interpreter::Interpreter::new(&mut lex);
    assert_eq!(15, my_interpreter.expr())
}