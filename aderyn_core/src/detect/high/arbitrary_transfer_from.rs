use std::collections::BTreeMap;
use std::error::Error;

use crate::ast::{Expression, FunctionCall, NodeID, TypeName};

use crate::capture;
use crate::detect::detector::IssueDetectorNamePool;
use crate::{
    context::workspace_context::WorkspaceContext,
    detect::detector::{IssueDetector, IssueSeverity},
};
use eyre::Result;

#[derive(Default)]
pub struct ArbitraryTransferFromDetector {
    // Keys are: [0] source file name, [1] line number, [2] character location of node.
    // Do not add items manually, use `capture!` to add nodes to this BTreeMap.
    found_instances: BTreeMap<(String, usize, String), NodeID>,
}

// Check if the first argument of the function call is valid
// In function calls with 3 args, the first arg [0] is the `from` address
// In function calls with 4 args, the second arg [1] is the `from` address
fn check_argument_validity(function_call: &FunctionCall) -> bool {
    let arg_index = if function_call.arguments.len() == 3 {
        0
    } else if function_call.arguments.len() == 4 {
        1
    } else {
        return false;
    };

    match &function_call.arguments[arg_index] {
        Expression::MemberAccess(arg_member_access) => {
            !(arg_member_access.member_name == "sender"
                && matches!(&*arg_member_access.expression, Expression::Identifier(identifier) if identifier.name == "msg"))
        }
        Expression::FunctionCall(arg_function_call) => {
            !(matches!(&*arg_function_call.expression, Expression::ElementaryTypeNameExpression(arg_el_type_name_exp) if matches!(&arg_el_type_name_exp.type_name, TypeName::ElementaryTypeName(type_name) if type_name.name == "address"))
                && matches!(arg_function_call.arguments.first(), Some(Expression::Identifier(arg_identifier)) if arg_identifier.name == "this"))
        }
        _ => true,
    }
}

impl IssueDetector for ArbitraryTransferFromDetector {
    fn detect(&mut self, context: &WorkspaceContext) -> Result<bool, Box<dyn Error>> {
        let transfer_from_function_calls =
            context
                .function_calls()
                .into_iter()
                .filter(|&function_call| {
                    // For each function call, check if the function call is a member access
                    // and if the member name is "transferFrom" or "safeTransferFrom", then check if the first argument is valid
                    // If the first argument is valid, add the function call to found_instances
                    if let Expression::MemberAccess(member_access) = &*function_call.expression {
                        if member_access.member_name == "transferFrom"
                            || member_access.member_name == "safeTransferFrom"
                        {
                            return check_argument_validity(function_call);
                        }
                    }
                    false
                });

        for item in transfer_from_function_calls {
            capture!(self, context, item);
        }

        Ok(!self.found_instances.is_empty())
    }

    fn severity(&self) -> IssueSeverity {
        IssueSeverity::High
    }

    fn title(&self) -> String {
        String::from("Arbitrary `from` passed to `transferFrom` (or `safeTransferFrom`)")
    }

    fn description(&self) -> String {
        String::from("Passing an arbitrary `from` address to `transferFrom` (or `safeTransferFrom`) can lead to loss of funds, because anyone can transfer tokens from the `from` address if an approval is made.  ")
    }

    fn instances(&self) -> BTreeMap<(String, usize, String), NodeID> {
        self.found_instances.clone()
    }

    fn name(&self) -> String {
        format!("{}", IssueDetectorNamePool::ArbitraryTransferFrom)
    }
}

#[cfg(test)]
mod arbitrary_transfer_from_tests {
    use serial_test::serial;

    use crate::detect::{
        detector::IssueDetector, high::arbitrary_transfer_from::ArbitraryTransferFromDetector,
    };

    #[test]
    #[serial]
    fn test_arbitrary_transfer_from_detector_by_loading_contract_directly() {
        let context = crate::detect::test_utils::load_solidity_source_unit(
            "../tests/contract-playground/src/ArbitraryTransferFrom.sol",
        );

        let mut detector = ArbitraryTransferFromDetector::default();
        let found = detector.detect(&context).unwrap();
        // assert that the detector found an issue
        assert!(found);
        // assert that the detector found the correct number of instances
        assert_eq!(detector.instances().len(), 4);
        // assert the severity is high
        assert_eq!(
            detector.severity(),
            crate::detect::detector::IssueSeverity::High
        );
        // assert the title is correct
        assert_eq!(
            detector.title(),
            String::from("Arbitrary `from` passed to `transferFrom` (or `safeTransferFrom`)")
        );
        // assert the description is correct
        assert_eq!(detector.description(), String::from("Passing an arbitrary `from` address to `transferFrom` (or `safeTransferFrom`) can lead to loss of funds, because anyone can transfer tokens from the `from` address if an approval is made.  "));
    }
}
