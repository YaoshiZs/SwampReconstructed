use crate::compilers::analyzer::ast::node::{ASTNode, ASTVec, ExprNode, ImportNode, ModuleType, VariableNode};
use crate::compilers::analyzer::{array_literal, class_definition, function_definition, instantiation, lazy_expr, map};
use crate::compilers::analyzer::composer::compose;
use crate::compilers::analyzer::symbol_priority::compare;
use crate::compilers::tokenizer::token::{Divider, Keyword, Paren, Token, TokenVec};
use crate::core::err::{assignment_error, import_error, internal_error, syntax_error, InternalComponent};
use crate::core::Symbols;

pub fn resolve(tokens: &mut TokenVec) -> Result<ExprNode, ()> {
    let mut params = ASTVec::new();

    while let Some(token) = tokens.pop_front() {
        match token {
            Token::Number(num) => params.push(ASTNode::Number(num)),
            Token::String(str) => params.push(ASTNode::String(str)),
            Token::Symbol(sym) => {
                if sym == Symbols::Equal {
                    return Err(assignment_error("invalid left-hand value")?);
                }
                params.push(ASTNode::Symbol(sym))
            }

            Token::Paren(paren) => {
                if paren == Paren::LeftBrace {
                    // lazy-expression && map definition

                    let mut is_map_definition = false;
                    for token in tokens.iter() {
                        if let Token::Divider(Divider::Colon) = token {
                            is_map_definition = true;
                            break;
                        }
                    }

                    if !is_map_definition {
                        let lazy_expression_node = lazy_expr::resolve(tokens)?;
                        params.push(ASTNode::LazyExpr(lazy_expression_node.into()));
                    } else {
                        let map_literal_node = map::resolve(tokens)?;
                        params.push(ASTNode::MapLiteral(map_literal_node.into()))
                    }
                } else if paren == Paren::LeftBracket {
                    // array literal
                    let array_literal_node = array_literal::resolve(tokens)?;
                    params.push(ASTNode::ArrayLiteral(array_literal_node.into()));
                } else if paren == Paren::LeftParen {
                    // nested expression
                    let current_node = resolve(tokens)?.into();
                    params.push(ASTNode::Expr(current_node));
                } else
                // breaks when encount `)`
                if paren == Paren::RightParen {
                    break;
                }
            }
            Token::Id(name) => {
                // variable
                // || function invocation
                // || array element reading
                // || map element reading
                // as compose
                let compose_node =
                    compose::resolve(ASTNode::Variable(VariableNode { name }.into()), tokens)?;
                params.push(compose_node);
            }

            Token::Keyword(Keyword::Import) => {
                let Some(next_token) = tokens.pop_front() else {
                    return Err(import_error("module name missing")?);
                };

                let Token::String(module_path) = next_token else {
                    return Err(import_error("invalid module name")?);
                };
                let node = ImportNode {
                    type__: ModuleType::UserDefined,
                    target: module_path,
                };
                params.push(ASTNode::ImportStatement(node.into()))
            }
            Token::Keyword(Keyword::Function) => {
                // function definition
                let function_definition = function_definition::resolve(tokens)?;
                params.push(ASTNode::FunctionDefinition(function_definition.into()));
            }
            Token::Keyword(Keyword::Class) => {
                // class definition
                let class_definition = class_definition::resolve(tokens)?;
                params.push(ASTNode::ClassDefinition(class_definition.into()));
            }
            Token::Keyword(Keyword::New) => {
                // class instantiation
                let instantiation_node = instantiation::resolve(tokens)?;
                params.push(ASTNode::Instantiation(instantiation_node.into()));
            }

            _ => {
                let msg = format!("unexpected expression token {}", token);
                return Err(syntax_error(&msg)?);
            }
        }
    }

    let mut symbol_stack = ASTVec::new();
    let mut result_stack = ASTVec::new();

    for node in params {
        match node {
            // regard the following ASTNode as number
            ASTNode::Variable(_)
            | ASTNode::Assignment(_)
            | ASTNode::Number(_)
            | ASTNode::String(_)
            | ASTNode::ArrayLiteral(_)
            | ASTNode::MapLiteral(_)
            | ASTNode::Expr(_)
            | ASTNode::Invocation(_)
            | ASTNode::LazyExpr(_)
            | ASTNode::Instantiation(_)
            | ASTNode::ObjectReading(_)
            | ASTNode::ImportStatement(_)
            | ASTNode::ClassDefinition(_)
            | ASTNode::FunctionDefinition(_)
            | ASTNode::ElementReading(_) => result_stack.push(node),

            ASTNode::Symbol(_) => {
                if symbol_stack.len() == 0 {
                    symbol_stack.push(node);
                    continue;
                }
                let current_node = &node;
                let mut last_node = symbol_stack.last().unwrap();
                let mut priority = compare(current_node, last_node)?;

                if priority > 1 {
                    // current priority > last priority
                    symbol_stack.push(node);
                } else {
                    while priority <= 0 {
                        let poped_node = symbol_stack.pop().unwrap();
                        result_stack.push(poped_node);

                        let optional_last = symbol_stack.last();
                        if optional_last.is_none() {
                            break;
                        }

                        last_node = optional_last.unwrap();
                        priority = compare(current_node, last_node)?;
                    }
                    symbol_stack.push(current_node.to_owned());
                }
            }
            _ => {
                let msg = format!("invalid expression: unexpected ASTNodeType: {}", node);
                return Err(internal_error(InternalComponent::Analyzer, &msg)?);
            }
        }
    }

    // pop the remain elements in the symbol_stack
    // and push them into the result_stack
    while symbol_stack.len() > 0 {
        let last_symbol_node = symbol_stack.pop().unwrap();
        result_stack.push(last_symbol_node);
    }

    Ok(ExprNode {
        elements: result_stack,
    })
}
