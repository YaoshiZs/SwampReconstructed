use crate::compilers::analyzer::ast::node::ASTNode;
use crate::compilers::analyzer::{expr_resolver, statement};
use crate::compilers::tokenizer::token::{Token, TokenVec};

pub fn resolve(tokens: &mut TokenVec) -> Result<ASTNode, ()> {
    if tokens.len() == 0 {
        // blank line || line comment
        Ok(ASTNode::Comment)
    } else if let Token::Keyword(keyword) = tokens[0] {
        // if matches keyword,
        // regard the whole sequence as a statement
        let statement_nodes = statement::resolve(keyword, tokens)?;
        Ok(ASTNode::Statement(statement_nodes.into()))
    } else {
        // regard the whole sequence as an expression
        let expression_nodes = expr_resolver::resolve(tokens)?;
        Ok(ASTNode::Expr(expression_nodes.into()))
    }
}
