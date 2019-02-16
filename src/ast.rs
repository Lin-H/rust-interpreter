// AST node
use super::token::Token;

pub struct AST {
    pub children: Vec<AST>,
    pub token: Token
}

impl AST {
    pub fn new(token: Token, children: Vec<AST>) -> AST {
        AST {
            token: token,
            children: children
        }
    }
}