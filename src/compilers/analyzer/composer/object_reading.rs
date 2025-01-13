
use crate::compilers::analyzer::ast::node::{ASTNode, ObjectReadingNode};
use crate::compilers::tokenizer::token::{Token, TokenVec};
use crate::core::err::syntax_error;


pub fn resolve(obj_node: ASTNode, tokens: &mut TokenVec) -> Result<ObjectReadingNode, ()> {
    // object property / method reading

    let Some(Token::Id(property)) =
        tokens.pop_front() else {
        return Err(syntax_error("missing object property")?)
    };

    Ok(ObjectReadingNode { obj_node, property })
}
