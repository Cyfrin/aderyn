use std::collections::BTreeMap;
use std::error::Error;

use crate::ast::Expression;
use crate::visitor::ast_visitor::Node;
use crate::{
    ast::MemberAccess,
    context::loader::{ASTNode, ContextLoader},
    detect::detector::{Detector, IssueSeverity},
    visitor::ast_visitor::ASTConstVisitor,
};
use eyre::Result;

#[derive(Default)]
pub struct ArbitraryTransferFromDetector {
    // Keys are source file name and line number
    found_instances: BTreeMap<(String, usize), String>,
}

impl Detector for ArbitraryTransferFromDetector {
    fn detect(&mut self, loader: &ContextLoader) -> Result<bool, Box<dyn Error>> {
        // Get all FunctionCalls where the expression is a MemberAccess with the member name "transferFrom",
        let function_calls = loader.function_calls.keys().filter(|function_call| {
            if let Expression::MemberAccess(member_access) = &*function_call.expression {
                if member_access.member_name == "transferFrom" {
                    return true;
                }
            }
            return false;
        });
        println!("function_calls: {:?}", function_calls.count());

        // For each function_call, If the first argument is not a MemberAccess with the member name "sender",
        // with an Identifier as the expression, with the name "msg", add the FunctionCall to the list of found instances.
        for function_call in loader.function_calls.keys() {
            if let Expression::MemberAccess(member_access) = &*function_call.expression {
                if member_access.member_name == "transferFrom" {
                    match &function_call.arguments[0] {
                        Expression::MemberAccess(member_access) => {
                            if member_access.member_name != "sender" {
                                if let Expression::Identifier(identifier) =
                                    &*member_access.expression
                                {
                                    if identifier.name != "msg" {
                                        self.found_instances.insert(
                                            loader.get_node_sort_key(&ASTNode::FunctionCall(
                                                function_call.clone(),
                                            )),
                                            function_call.src.clone(),
                                        );
                                    }
                                }
                            }
                        }
                        _ => {
                            self.found_instances.insert(
                                loader.get_node_sort_key(&ASTNode::FunctionCall(
                                    function_call.clone(),
                                )),
                                function_call.src.clone(),
                            );
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
        String::from("")
    }

    fn description(&self) -> String {
        String::from("")
    }

    fn instances(&self) -> BTreeMap<(String, usize), String> {
        self.found_instances.clone()
    }
}

#[cfg(test)]
mod arbitrary_transfer_from_tests {
    use crate::detect::{
        detector::{detector_test_helpers::load_contract, Detector},
        high::arbitrary_transfer_from::ArbitraryTransferFromDetector,
    };

    #[test]
    fn test_arbitrary_transfer_from_detector() {
        let context_loader = load_contract(
            "../tests/contract-playground/out/ArbitraryTransferFrom.sol/ArbitraryTransferFrom.json",
        );
        let mut detector = ArbitraryTransferFromDetector::default();
        let found = detector.detect(&context_loader).unwrap();
        // assert that the detector found an issue
        assert!(found);
        // assert that the detector found the correct number of instances
        assert_eq!(detector.instances().len(), 1);
        // assert the severity is high
        assert_eq!(
            detector.severity(),
            crate::detect::detector::IssueSeverity::High
        );
        // assert the title is correct
        assert_eq!(detector.title(), String::from(""));
        // assert the description is correct
        assert_eq!(detector.description(), String::from(""));
    }
}
