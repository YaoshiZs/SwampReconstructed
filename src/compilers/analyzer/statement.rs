use crate::compilers::analyzer::ast::node::{ExprNode, ForStatement, IfStatement, ImportNode, ModuleType, StatementNode};
use crate::compilers::analyzer::{expr_resolver, statement_block};
use crate::compilers::tokenizer::token::{Keyword, Paren, Token, TokenVec};
use crate::core::err::{import_error, syntax_error};

fn statement_condition_resolve(tokens: &mut TokenVec) -> Result<ExprNode, ()> {
    let mut sub_tokens = TokenVec::new(); // sub condition tokens

    while let Some(token) = tokens.pop_front() {
        if token == Token::Paren(Paren::LeftBrace) {
            break;
        }
        sub_tokens.push_back(token);
    }
    Ok(expr_resolver::resolve(&mut sub_tokens)?)
}

pub fn resolve(keyword: Keyword, tokens: &mut TokenVec) -> Result<StatementNode, ()> {
    // remove the keyword token
    tokens.pop_front();

    let result = match keyword {
        Keyword::Out => {
            let output_expression = expr_resolver::resolve(tokens)?;
            StatementNode::Output(output_expression)
        }
        Keyword::For => StatementNode::ForLoop(ForStatement {
            loop_count: statement_condition_resolve(tokens)?,
            body: statement_block::resolve(tokens)?,
        }),
        Keyword::If => StatementNode::Condition(IfStatement {
            condition: statement_condition_resolve(tokens)?,
            body: statement_block::resolve(tokens)?,
        }),

        Keyword::Import => {
            let Some(next_token) = tokens.pop_front() else {
                return Err(())
            };
            let Token::Id(module_name) = next_token else {
                return Err(import_error("invalid module name")?);
            };
            let node = ImportNode {
                type__: ModuleType::BuildIn,
                target: module_name,
            };
            StatementNode::Import(node)
        }


        Keyword::Break => StatementNode::Break(expr_resolver::resolve(tokens)?),
        Keyword::Continue => StatementNode::Continue, // Do nothing
        _ => {
            // example:
            // if 1 {new}
            let msg = format!("unexpected keyword '{}' at start of statement", keyword);
            return Err(syntax_error(&msg)?);
        }
    };
    return Ok(result);
}
