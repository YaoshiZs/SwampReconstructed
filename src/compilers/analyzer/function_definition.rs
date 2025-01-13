use crate::compilers::analyzer::ast::node::FunctionDefinitionNode;
use crate::compilers::tokenizer::token::{Divider, Paren, Token, TokenVec};
use crate::core::err::syntax_error;
use crate::core::value::references::function::UserDefinedFnParam;
use crate::core::value::value::ValueType;
use super::statement_block;

// refactor: params_resolve
fn params_resolve(tokens: &mut TokenVec) -> Result<Vec<UserDefinedFnParam>, ()> {
    // structure:
    // id annotation) {function body ...}

    let mut params = vec![];

    while let Some(current) = tokens.pop_front() {
        match current {
            Token::Id(id) => {
                let Some(next) = tokens.pop_front() else {
                    return Err(syntax_error(
                        "incomplete function definition",
                    )?);
                };
                if let Token::Annotation(type__) = next {
                    params.push(UserDefinedFnParam { type__, id })
                } else if let Token::Divider(Divider::Comma) | Token::Paren(Paren::RightParen) =
                    next
                {
                    tokens.push_front(next);
                    params.push(UserDefinedFnParam {
                        type__: ValueType::Void,
                        id,
                    });
                } else {
                    return Err(syntax_error(
                        "type annotation expected in function definition",
                    )?);
                }
            }
            Token::Divider(Divider::Comma) => continue,
            Token::Paren(Paren::RightParen) => break,
            _ => {
                let msg = format!("unexpected token {} in function param", current);
                return Err(syntax_error(&msg)?);
            }
        }
    }
    Ok(params)
}

pub fn resolve(tokens: &mut TokenVec) -> Result<FunctionDefinitionNode, ()> {
    // no `fn` keyword
    // example:
    // (param $_) {out param}

    if tokens.len() == 0 {
        return Err(syntax_error("missing function definition")?);
    }

    let first_token = tokens.pop_front().unwrap();
    if first_token == Token::Paren(Paren::LeftParen) {
        let function_params = params_resolve(tokens)?;

        let next_token = tokens.pop_front();
        if next_token != Some(Token::Paren(Paren::LeftBrace)) {
            return Err(syntax_error("missing function body, expected '{'")?);
        }

        let function_body = statement_block::resolve(tokens)?;

        Ok(FunctionDefinitionNode {
            params: function_params,
            name: None,
            body: function_body,
        })
    } else {
        Err(syntax_error(
            "missing function param definition, expected '('",
        )?)
    }
}
