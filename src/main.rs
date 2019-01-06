mod interpreter;
mod lexer;
#[allow(dead_code)]
mod token;

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
