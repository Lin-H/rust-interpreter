use super::token::TokenType;
use super::parser::Parser;
use super::ast::AST;

pub struct Interpreter<'a> {
    parser: &'a mut Parser<'a>,
}

impl<'a> Interpreter<'a> {
    pub fn new(parser: &'a mut Parser<'a>) -> Interpreter<'a> {
        Interpreter {
            parser: parser
        }
    }

    pub fn interpret(self) -> i32 {
        let a = &(self.parser.parse());
        self.visit(a)
    }

    pub fn visit(&self, tree: &AST) -> i32 {
        match tree.token.token {
            TokenType::INTEGER => self.visit_num(tree),
            TokenType::PLUS | TokenType::MINUS | TokenType::MUL | TokenType::DIV => {
                return if tree.children.len() == 1 { self.visit_unary(tree) } else { self.visit_binop(tree) }
            },
            _ => panic!("visit error")
        }
    }

    fn visit_num(&self, num: &AST) -> i32 {
        num.token.value.parse().unwrap()
    }
    fn visit_binop(&self, node: &AST) -> i32 {
        match node.token.token {
            TokenType::PLUS => {
                return self.visit(&node.children[0]) + self.visit(&node.children[1])
            }
            TokenType::MINUS => {
                return self.visit(&node.children[0]) - self.visit(&node.children[1])
            }
            TokenType::MUL => {
                return self.visit(&node.children[0]) * self.visit(&node.children[1])
            }
            TokenType::DIV => {
                return self.visit(&node.children[0]) / self.visit(&node.children[1])
            },
            _ => panic!("visit_binop error")
        }
    }
    fn visit_unary(&self, node: &AST) -> i32 {
        if node.token.token == TokenType::MINUS {
            -self.visit(&node.children[0])
        } else {
            self.visit(&node.children[0])
        }
    }
}