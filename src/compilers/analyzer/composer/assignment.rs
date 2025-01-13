use crate::compilers::analyzer::ast::node::{ASTNode, AssignmentNode, ExprNode};
use crate::compilers::tokenizer::token::TokenVec;
use crate::core::err::assignment_error;
use crate::core::Symbols;
use super::super::expr_resolver;

pub fn resolve(
    tokens: &mut TokenVec,
    equal_symbol: Symbols,
    left_hand_node: ASTNode,
) -> Result<AssignmentNode, ()> {
    // assignment
    // `symbol` may be: += | -= | *= | /= | ^=

    let mut right_hand_node = expr_resolver::resolve(tokens)?;

    if right_hand_node.elements.len() == 0 {
        // example:
        // var =
        return Err(assignment_error("missing right-hand value")?);
    }

    if equal_symbol != Symbols::Equal {
        let origin_node = ASTNode::Expr(right_hand_node.into());
        let separated = equal_symbol.separate();
        let variable_node = left_hand_node.clone();
        let symbol_node = ASTNode::Symbol(separated);

        right_hand_node = ExprNode {
            elements: vec![variable_node, origin_node, symbol_node],
        };
    }

    let current_node = AssignmentNode {
        left_hand_node,
        right_hand_node,
    };

    Ok(current_node)
}
