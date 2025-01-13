use std::rc::Rc;
use crate::compilers::analyzer::ast::node::{ClassDefinitionNode, FunctionDefinitionNode};
use crate::compilers::tokenizer::token::{Divider, Paren, Token, TokenVec};

use crate::core::err::syntax_error;
use crate::core::Symbols;
use crate::core::value::references::class::Property;
use crate::core::value::references::function::UserDefinedFnParam;
use crate::core::value::value::ValueType;
use super::function_definition;

pub fn resolve(tokens: &mut TokenVec) -> Result<ClassDefinitionNode, ()> {
    // no `cl` keyword
    // example:
    // { prop $_, method=(self $_){do something...} }

    if tokens.len() == 0 {
        return Err(syntax_error("missing class body")?);
    }

    let mut properties = Vec::<Property>::new();
    let mut method_nodes = Vec::<Rc<FunctionDefinitionNode>>::new();

    let first_token = tokens.pop_front().unwrap();

    if first_token == Token::Paren(Paren::LeftBrace) {
        loop {
            if tokens.len() == 0 {
                return Err(syntax_error("unmatched brace")?);
            }

            let current = tokens.pop_front().unwrap();

            if let Token::Id(id) = current {
                let Some(next_token) = tokens.pop_front() else {
                    // if no token follows the property
                    return Err(syntax_error("unmatched brace")?)
                };

                match next_token {
                    Token::Annotation(type__) => properties.push(Property(type__, id)),
                    Token::Symbol(Symbols::Equal) => {
                        // current as class method
                        let mut method_node = function_definition::resolve(tokens)?;
                        method_node.params.insert(
                            0,
                            UserDefinedFnParam {
                                type__: ValueType::Object,
                                id: String::from("self"),
                            },
                        );
                        method_node.name = Some(id);
                        method_nodes.push(method_node.into())
                    }
                    _ => {
                        let msg = format!("unexpected token {} in class body", next_token);
                        return Err(syntax_error(&msg)?);
                    }
                }
            } else if current == Token::Divider(Divider::Semicolon) {
                continue;
            } else if current == Token::Paren(Paren::RightBrace) {
                break;
            } else {
                let msg = format!("unexpected token {} in class body", current);
                return Err(syntax_error(&msg)?);
            }
        }
    } else {
        return Err(syntax_error("expected class-definition body")?);
    }
    Ok(ClassDefinitionNode {
        properties,
        method_nodes,
    })
}
