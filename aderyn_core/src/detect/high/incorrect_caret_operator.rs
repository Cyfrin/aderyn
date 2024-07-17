use std::collections::BTreeMap;
use std::error::Error;

use crate::ast::{Expression, LiteralKind, Mutability, NodeID};

use crate::capture;
use crate::context::workspace_context::ASTNode;
use crate::detect::detector::IssueDetectorNamePool;
use crate::{
    context::workspace_context::WorkspaceContext,
    detect::detector::{IssueDetector, IssueSeverity},
};
use eyre::Result;

#[derive(Default)]
pub struct IncorrectUseOfCaretOperatorDetector {
    // Keys are: [0] source file name, [1] line number, [2] character location of node.
    // Do not add items manually, use `capture!` to add nodes to this BTreeMap.
    found_instances: BTreeMap<(String, usize, String), NodeID>,
}

impl IssueDetector for IncorrectUseOfCaretOperatorDetector {
    fn detect(&mut self, context: &WorkspaceContext) -> Result<bool, Box<dyn Error>> {
        // Copied Heuristic from Slither:
        // look for binary expressions with ^ operator where at least one of the operands is a constant, and
        // # the constant is not in hex, because hex typically is used with bitwise xor and not exponentiation

        println!("{:?}", context.binary_operations());

        for binary_operation in context
            .binary_operations()
            .into_iter()
            .filter(|op| op.operator == "^")
        {
            for expr in [
                binary_operation.left_expression.as_ref(),
                binary_operation.right_expression.as_ref(),
            ] {
                if let Expression::Literal(literal) = expr {
                    if literal.kind == LiteralKind::Number
                        && literal.value.as_ref().is_some_and(|v| !v.starts_with("0x"))
                    {
                        capture!(self, context, binary_operation);
                    }
                }
                if let Expression::Identifier(identifier) = expr {
                    if let Some(ref_decl) = identifier.referenced_declaration {
                        if let Some(ASTNode::VariableDeclaration(v)) = context.nodes.get(&ref_decl)
                        {
                            if v.mutability() == Some(&Mutability::Constant) {
                                if let Some(Expression::Literal(literal)) = v.value.as_ref() {
                                    if literal.kind == LiteralKind::Number
                                        && literal
                                            .value
                                            .as_ref()
                                            .is_some_and(|v| !v.starts_with("0x"))
                                    {
                                        capture!(self, context, binary_operation);
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }

        Ok(!self.found_instances.is_empty())
    }

    fn severity(&self) -> IssueSeverity {
        IssueSeverity::High
    }

    fn title(&self) -> String {
        String::from("Incorrect use of caret operator on a non hexadcimal constant")
    }

    fn description(&self) -> String {
        String::from("The caret operator is usually mistakenly thought of as an exponentiation operator but actually, it's a bitwise xor operator.")
    }

    fn instances(&self) -> BTreeMap<(String, usize, String), NodeID> {
        self.found_instances.clone()
    }

    fn name(&self) -> String {
        IssueDetectorNamePool::IncorrectCaretOperator.to_string()
    }
}

#[cfg(test)]
mod incorrect_use_of_caret_operator_tests {
    use crate::detect::{detector::IssueDetector, high::IncorrectUseOfCaretOperatorDetector};

    #[test]
    fn test_incorrect_use_of_operator_detector() {
        let context = crate::detect::test_utils::load_solidity_source_unit(
            "../tests/contract-playground/src/IncorrectCaretOperator.sol",
        );

        let mut detector = IncorrectUseOfCaretOperatorDetector::default();
        let found = detector.detect(&context).unwrap();

        println!("{:#?}", detector.instances());

        // assert that the detector found an issue
        assert!(found);
        // assert that the detector found the correct number of instances
        assert_eq!(detector.instances().len(), 5);
        // assert the severity is high
        assert_eq!(
            detector.severity(),
            crate::detect::detector::IssueSeverity::High
        );
        // assert the title is correct
        assert_eq!(
            detector.title(),
            String::from("Incorrect use of caret operator on a non hexadcimal constant")
        );
        // assert the description is correct
        assert_eq!(detector.description(), String::from("The caret operator is usually mistakenly thought of as an exponentiation operator but actually, it's a bitwise xor operator."));
    }
}
