use crate::compilers::analyzer::ast::node::LazyExprNode;
use crate::compilers::analyzer::sequence;
use crate::compilers::tokenizer::token::{Paren, Token, TokenVec};
use crate::core::err::syntax_error;

pub fn resolve(tokens: &mut TokenVec) -> Result<LazyExprNode, ()> {
    let mut sub_tokens = TokenVec::new();
    let mut brace_count = 1;

    while let Some(token) = tokens.pop_front() {
        if token == Token::Paren(Paren::LeftBrace) {
            brace_count += 1;
        }
        if token == Token::Paren(Paren::RightBrace) {
            brace_count -= 1;
            if brace_count == 0 {
                break;
            }
        }
        sub_tokens.push_back(token);
    }
    if brace_count > 0 {
        return Err(syntax_error("unmatched brace")?);
    }

    let sub_sequence = sequence::resolve(&mut sub_tokens)?;
    Ok(LazyExprNode { sub_sequence })
}
