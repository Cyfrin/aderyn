use std::collections::BTreeMap;
use std::error::Error;

use crate::ast::{ASTNode, Expression, Identifier, MemberAccess, NodeID, NodeType};

use crate::capture;
use crate::context::browser::GetImmediateParent;
use crate::detect::detector::IssueDetectorNamePool;
use crate::{
    context::workspace_context::WorkspaceContext,
    detect::detector::{IssueDetector, IssueSeverity},
};
use eyre::Result;

#[derive(Default)]
pub struct UncheckedReturnDetector {
    // Keys are: [0] source file name, [1] line number, [2] character location of node.
    // Do not add items manually, use `capture!` to add nodes to this BTreeMap.
    found_instances: BTreeMap<(String, usize, String), NodeID>,
}

impl IssueDetector for UncheckedReturnDetector {
    fn detect(&mut self, context: &WorkspaceContext) -> Result<bool, Box<dyn Error>> {
        // When you have found an instance of the issue,
        // use the following macro to add it to `found_instances`:
        //
        // capture!(self, context, item);

        for function_call in context.function_calls() {
            // Find the ID of FunctionDefinition that we're calling so that we may identify if there are returned params
            match function_call.expression.as_ref() {
                Expression::Identifier(Identifier {
                    referenced_declaration: Some(id),
                    ..
                })
                | Expression::MemberAccess(MemberAccess {
                    referenced_declaration: Some(id),
                    ..
                }) => {
                    if function_call
                        .parent(context)
                        .is_some_and(|node| node.node_type() == NodeType::Block)
                    {
                        // Now, we know that the return value is unused
                        if let Some(ASTNode::FunctionDefinition(function)) = context.nodes.get(id) {
                            if !function.return_parameters.parameters.is_empty() {
                                // Now, we know that the function has no return value
                                capture!(self, context, function_call);
                            }
                        }
                    }
                }
                _ => (),
            };
        }

        Ok(!self.found_instances.is_empty())
    }

    fn severity(&self) -> IssueSeverity {
        IssueSeverity::High
    }

    fn title(&self) -> String {
        String::from("Return value of the function call is not checked.")
    }

    fn description(&self) -> String {
        String::from("Function returns a value but it is ignored.")
    }

    fn instances(&self) -> BTreeMap<(String, usize, String), NodeID> {
        self.found_instances.clone()
    }

    fn name(&self) -> String {
        IssueDetectorNamePool::UncheckedReturn.to_string()
    }
}

#[cfg(test)]
mod unchecked_return_tests {
    use serial_test::serial;

    use crate::detect::{detector::IssueDetector, high::unchecked_return::UncheckedReturnDetector};

    #[test]
    #[serial]
    fn test_unchecked_return_detector() {
        let context = crate::detect::test_utils::load_solidity_source_unit(
            "../tests/contract-playground/src/UncheckedReturn.sol",
        );

        let mut detector = UncheckedReturnDetector::default();
        let found = detector.detect(&context).unwrap();
        // assert that the detector found an issue

        println!("{:?}", detector.instances());

        assert!(found);
        // assert that the detector found the correct number of instances
        assert_eq!(detector.instances().len(), 2);
        // assert the severity is high
        assert_eq!(
            detector.severity(),
            crate::detect::detector::IssueSeverity::High
        );
        // assert the title is correct
        assert_eq!(
            detector.title(),
            String::from("Return value of the function call is not checked.")
        );
        // assert the description is correct
        assert_eq!(
            detector.description(),
            String::from("Function returns a value but it is ignored.")
        );
    }
}
