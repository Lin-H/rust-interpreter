mod interpreter;
mod lexer;
#[allow(dead_code)]
mod token;
mod parser;
mod ast;

use std::io::{self, Write};

fn main() {
    let mut input = String::new();
    loop {
        print!("calc>");
        io::stdout().flush().unwrap();
        io::stdin().read_line(&mut input).unwrap();
        let mut lex = lexer::Lexer::new(&input);
        let parser = &mut parser::Parser::new(&mut lex);
        let my_interpreter = interpreter::Interpreter::new(parser);
        println!("{:?}", my_interpreter.interpret());
        input.clear();
    }
}

#[test]
fn add() {
    let mut lex = lexer::Lexer::new("1+1\r\n");
    let parser = &mut parser::Parser::new(&mut lex);
    let my_interpreter = interpreter::Interpreter::new(parser);
    assert_eq!(2, my_interpreter.interpret());
}
#[test]
fn minus() {
    let mut lex = lexer::Lexer::new("10-1\r\n");
    let parser = &mut parser::Parser::new(&mut lex);
    let my_interpreter = interpreter::Interpreter::new(parser);
    assert_eq!(9, my_interpreter.interpret());
}
#[test]
fn mul() {
    let mut lex = lexer::Lexer::new("7*3\r\n");
    let parser = &mut parser::Parser::new(&mut lex);
    let my_interpreter = interpreter::Interpreter::new(parser);
    assert_eq!(21, my_interpreter.interpret());
}

#[test]
fn div() {
    let mut lex = lexer::Lexer::new("8/2\r\n");
    let parser = &mut parser::Parser::new(&mut lex);
    let my_interpreter = interpreter::Interpreter::new(parser);
    assert_eq!(4, my_interpreter.interpret());
}

#[test]
fn paren() {
    let mut lex = lexer::Lexer::new("10 * (2+4) / 4\r\n");
    let parser = &mut parser::Parser::new(&mut lex);
    let my_interpreter = interpreter::Interpreter::new(parser);
    assert_eq!(15, my_interpreter.interpret())
}

#[test]
fn single() {
    let mut lex = lexer::Lexer::new("10\r\n");
    let parser = &mut parser::Parser::new(&mut lex);
    let my_interpreter = interpreter::Interpreter::new(parser);
    assert_eq!(10, my_interpreter.interpret())
}

#[test]
#[should_panic]
fn panic() {
    let mut lex = lexer::Lexer::new("10 * (2+4) / \r\n");
    let parser = &mut parser::Parser::new(&mut lex);
    let my_interpreter = interpreter::Interpreter::new(parser);
    assert_eq!(15, my_interpreter.interpret())
}