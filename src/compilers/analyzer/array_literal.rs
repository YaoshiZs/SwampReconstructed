
use crate::compilers::analyzer::ast::node::ArrayLiteralNode;
use crate::compilers::tokenizer::token::{Paren, TokenVec};


use super::list;

pub fn resolve(tokens: &mut TokenVec) -> Result<ArrayLiteralNode, ()> {
    let elements = list::resolve(tokens, Paren::RightBracket)?;
    Ok(ArrayLiteralNode { elements })
}
