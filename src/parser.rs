use super::lexer::Lexer;
use super::token::Token;
use super::token::TokenType;
use super::ast::AST;

pub struct Parser<'a> {
    lexer: &'a mut Lexer<'a>,
    // set current token to the first token taken from the input
    current_token: Token
}

impl<'a> Parser<'a> {
    pub fn new(lexer: &'a mut Lexer<'a>) -> Parser<'a> {
        Parser {
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
    // factor : (PLUS | MINUS) factor | INTEGER | LPAREN expr RPAREN
    pub fn factor(&mut self) -> AST {
        let token = self.current_token.clone();
        match token.token {
            TokenType::PLUS | TokenType::MINUS => {
                self.eat(token.token);
                return AST::new(token, vec![self.factor()]);
            },
            TokenType::INTEGER => {
                self.eat(TokenType::INTEGER);
                return AST::new(token, vec![]);
            },
            TokenType::LPAREN => {
                self.eat(TokenType::LPAREN);
                let result = self.expr();
                self.eat(TokenType::RPAREN);
                return result;
            },
            _ => panic!("factor error")
        }
    }
    /*
    expr:
    calc> 7 + 3 * (10 / (12 / (3 + 1) - 1))
        22

        expr   : term ((PLUS | MINUS) term)*
        term   : factor ((MUL | DIV) factor)*
        factor : (PLUS | MINUS) factor | INTEGER | LPAREN expr RPAREN
     */
    pub fn expr(&mut self) -> AST {
        let mut node = self.term();
        loop {
            let current_token = self.current_token.clone();
            match self.current_token.token {
                TokenType::PLUS => {
                    self.eat(TokenType::PLUS);
                },
                TokenType::MINUS => {
                    self.eat(TokenType::MINUS);
                },
                _ => break
            }
            node = AST::new(current_token, vec![node, self.term()]);
        }
        node
    }

    // term   : factor ((MUL | DIV) factor)*
    pub fn term(&mut self) -> AST {
        let mut node = self.factor();
        loop {
            let current_token = self.current_token.clone();
            match current_token.token {
                TokenType::MUL => {
                    self.eat(TokenType::MUL);
                },
                TokenType::DIV => {
                    self.eat(TokenType::DIV);
                },
                _ => break
            }
            node = AST::new(current_token, vec![node, self.factor()])
        }
        node
    }

    pub fn parse(&mut self) -> AST {
        self.expr()
    }
}