use eyre::Result;

use crate::{
    ast::{BinaryOperation, Expression, FunctionDefinition, NodeID},
    visitor::ast_visitor::{ASTConstVisitor, Node},
};

pub struct BinaryChecks {
    pub checks: Vec<BinaryCheckStatement>,
}

pub struct BinaryCheckStatement {
    pub l_node_id: Option<NodeID>,
    pub r_node_id: Option<NodeID>,
    pub operator: String,
}

impl From<&FunctionDefinition> for BinaryChecks {
    fn from(function_definition: &FunctionDefinition) -> BinaryChecks {
        let mut binary_checks = Self { checks: vec![] };
        function_definition
            .accept(&mut binary_checks)
            .unwrap_or_default();
        binary_checks
    }
}

impl ASTConstVisitor for BinaryChecks {
    fn visit_binary_operation(&mut self, node: &BinaryOperation) -> Result<bool> {
        let operator = node.operator.clone();

        let l_node_id: Option<NodeID> = {
            let l = node.left_expression.as_ref();
            if let Expression::Identifier(left_identifier) = l {
                Some(left_identifier.referenced_declaration)
            } else {
                None
            }
        };

        let r_node_id: Option<NodeID> = {
            let r = node.right_expression.as_ref();
            if let Expression::Identifier(right_identifier) = r {
                Some(right_identifier.referenced_declaration)
            } else {
                None
            }
        };

        self.checks.push(BinaryCheckStatement {
            l_node_id,
            r_node_id,
            operator,
        });

        Ok(true)
    }
}
