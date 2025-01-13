use crate::compilers::analyzer::ast::node::{ASTNode, ElementReadingNode};
use crate::compilers::tokenizer::token::{Paren, Token, TokenVec};
use super::super::expr_resolver;

pub fn resolve(target_node: ASTNode, tokens: &mut TokenVec) -> Result<ElementReadingNode, ()> {
    // example for ArrayReading:
    // 1] | from `arr[1]`
    // 1][2] | from `arr[1][2]`
    // example for MapReading:
    // "prop"] from `map["prop"]`

    let mut bracket_count = 1;
    let mut sub_tokens = TokenVec::new();

    while let Some(token) = tokens.pop_front() {
        if token == Token::Paren(Paren::LeftBracket) {
            bracket_count += 1;
        }
        if token == Token::Paren(Paren::RightBracket) {
            bracket_count -= 1;
            if bracket_count == 0 {
                break;
            }
        }
        sub_tokens.push_back(token);
    }
    let index_node = expr_resolver::resolve(&mut sub_tokens)?;
    Ok(ElementReadingNode {
        target_node,
        index_node,
    })
}
