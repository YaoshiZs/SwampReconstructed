use crate::compilers::analyzer::ast::node::{ArrayLiteralNode, InstantiationNode};
use crate::compilers::tokenizer::token::{Paren, Token, TokenVec};
use crate::core::err::syntax_error;
use super::list;

pub fn resolve(tokens: &mut TokenVec) -> Result<InstantiationNode, ()> {
    // no `new` keyword
    // example:
    // Person["test", 99] | from `new Person["test", 99]`

    let Some(Token::Id(target_class)) =
        tokens.pop_front() else {
        return Err(syntax_error("missing class name")?)
    };

    // expect: `[`
    if tokens.pop_front() != Some(Token::Paren(Paren::LeftParen)) {
        return Err(syntax_error(
            "missing params for object instantiation, expected '['",
        )?);
    }

    let instantiation_params = list::resolve(tokens, Paren::RightParen)?;
    // let instantiation_params =
    //     array::literal_resolve(tokens)?;

    Ok(InstantiationNode {
        class: target_class,
        params: ArrayLiteralNode {
            elements: instantiation_params,
        },
    })
}