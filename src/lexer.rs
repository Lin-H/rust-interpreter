use std::str::Chars;

pub struct Lexer<'a> {
    // client string input
    text: Chars<'a>,
    pos: usize,
    current_char: char
}

impl<'a> Lexer<'a> {
    pub fn new(text: &'a str) -> Lexer<'a> {
        let mut chars: Chars = text.chars();
        Lexer {
            current_char: chars.nth(0).unwrap(),
            text: chars,
            pos: 0
        }
    }
    // Advance the `pos` pointer and set the `current_char` variable.
    pub fn advance(&mut self) {
        self.pos += 1;
        let current_char = self.text.nth(self.pos).unwrap();
        self.current_char = current_char;
    }
    pub fn skip_whitespace(&mut self) {
        while self.current_char == ' ' {
            self.advance()
        }
    }
    // Return a (multidigit) integer consumed from the input.
    pub fn integer(&mut self) -> u32 {
        let mut result: u32 = 0;
        while self.current_char.is_digit(10) {
            result += self.current_char.to_digit(10).unwrap();
            self.advance();
        }
        result
    }
}
