
use crate::compilers::analyzer::ast::node::{ASTNode, InvocationNode};
use crate::compilers::analyzer::list;
use crate::compilers::tokenizer::token::{Paren, TokenVec};


pub fn resolve(caller: ASTNode, tokens: &mut TokenVec) -> Result<InvocationNode, ()> {
    let params = list::resolve(tokens, Paren::RightParen)?;
    Ok(InvocationNode { caller, params })
}
