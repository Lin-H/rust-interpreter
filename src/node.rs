// AST node
use super::token::Token;

pub struct BinOp {
    pub left: Node,
    pub token: Token,
    pub right: Node
}

impl BinOp {
    pub fn new() -> BinOp {
    }
}