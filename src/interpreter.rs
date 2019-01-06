use super::lexer::Lexer;
use super::token::Token;
use super::token::TokenType;

pub struct Interpreter<'a> {
    lexer: &'a mut Lexer<'a>,
    // set current token to the first token taken from the input
    current_token: Token
}

impl<'a> Interpreter<'a> {
    pub fn new(lexer: &'a mut Lexer<'a>) -> Interpreter<'a> {
        Interpreter {
            current_token: lexer.get_next_token(),
            lexer: lexer
        }
    }
    /* compare the current token type with the passed token
       type and if they match then "eat" the current token
       and assign the next token to the self.current_token,
       otherwise raise an exception. */
    pub fn eat(&mut self, token_type: TokenType) {
        if self.current_token.token == token_type {
            self.current_token = self.lexer.get_next_token();
        } else {
            panic!("eat error")
        }
    }
    // factor : INTEGER | LPAREN expr RPAREN
    pub fn factor(&mut self) -> u32 {
        let token = &self.current_token;
        if token.token == TokenType::INTEGER {
            let result = token.value.parse::<u32>().unwrap();
            self.eat(TokenType::INTEGER);
            return result;
        } else if token.token == TokenType::LPAREN {
            self.eat(TokenType::LPAREN);
            let result = self.expr();
            self.eat(TokenType::RPAREN);
            return result;
        }
        panic!("factor error");
    }
    /*
    expr:
    calc> 7 + 3 * (10 / (12 / (3 + 1) - 1))
        22

        expr   : term ((PLUS | MINUS) term)*
        term   : factor ((MUL | DIV) factor)*
        factor : INTEGER | LPAREN expr RPAREN
     */
    pub fn expr(&mut self) -> u32 {
        let mut result = self.term();
        loop {
            match self.current_token.token {
                TokenType::PLUS => {
                    self.eat(TokenType::PLUS);
                    result += self.term();
                },
                TokenType::MINUS => {
                    self.eat(TokenType::MINUS);
                    result -= self.term();
                },
                _ => break
            }
        }
        return result;
    }

    // term   : factor ((MUL | DIV) factor)*
    pub fn term(&mut self) -> u32 {
        let mut result = self.factor();
        loop {
            match self.current_token.token {
                TokenType::MUL => {
                    self.eat(TokenType::MUL);
                    result *= self.factor();
                },
                TokenType::DIV => {
                    self.eat(TokenType::DIV);
                    result /= self.factor();
                },
                _ => break
            }
        }
        return result;
    }
}