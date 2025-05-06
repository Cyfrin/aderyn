use std::{collections::BTreeMap, error::Error};

use crate::{
    ast::{Expression, Identifier, NodeID},
    capture,
    context::{browser::ExtractFunctionCalls, workspace::WorkspaceContext},
    detect::{
        detector::{IssueDetector, IssueDetectorNamePool, IssueSeverity},
        helpers::{get_implemented_external_and_public_functions, has_msg_sender_binary_operation},
    },
};
use eyre::Result;

#[derive(Default)]
pub struct ArbitraryTransferFromDetector {
    // Keys are: [0] source file name, [1] line number, [2] character location of node.
    // Do not add items manually, use `capture!` to add nodes to this BTreeMap.
    found_instances: BTreeMap<(String, usize, String), NodeID>,
}

impl IssueDetector for ArbitraryTransferFromDetector {
    fn detect(&mut self, context: &WorkspaceContext) -> Result<bool, Box<dyn Error>> {
        // Applying devtooligan's suggestion
        // * Operate on public and external functions only
        // * See that msg.sender is not checked
        // * Check that the argument passed in is from the parameter list of the said function

        let suspected_functions =
            get_implemented_external_and_public_functions(context).filter(|function_definition| {
                !has_msg_sender_binary_operation(&((*function_definition).into()))
                    && function_definition.modifiers.is_empty() // If there are modifiers, assume
                                                                // the function is safe because
                                                                // sometime modifiers' definition
                                                                // may not be in scope
            });

        for func in suspected_functions {
            let func_parameters_ids =
                &func.parameters.parameters.iter().map(|f| f.id).collect::<Vec<_>>();

            let transfer_func_calls = ExtractFunctionCalls::from(func)
                .extracted
                .into_iter()
                .filter(|function_call| {
                    // For each function call, check if the function call is a member access
                    // and if the member name is "transferFrom" or "safeTransferFrom", then check if
                    // the first argument is valid If the first argument is
                    // valid, add the function call to found_instances
                    if let Expression::MemberAccess(member_access) = &*function_call.expression {
                        if member_access.member_name == "transferFrom"
                            || member_access.member_name == "safeTransferFrom"
                        {
                            return true;
                        }
                    }
                    false
                })
                .collect::<Vec<_>>();

            for func in transfer_func_calls {
                // Check if the first argument of the function call is valid
                // In function calls with 3 args, the first arg [0] is the `from` address
                // In function calls with 4 args, the second arg [1] is the `from` address
                let arg_index = if func.arguments.len() == 3 {
                    0
                } else if func.arguments.len() == 4 {
                    1
                } else {
                    continue;
                };

                let arg = &func.arguments[arg_index];

                if let Expression::Identifier(Identifier {
                    referenced_declaration: Some(referenced_id),
                    ..
                }) = arg
                {
                    if func_parameters_ids.iter().any(|r| r == referenced_id) {
                        capture!(self, context, func);
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
        String::from("Arbitrary `from` Passed to `transferFrom`")
    }

    fn description(&self) -> String {
        String::from("Passing an arbitrary `from` address to `transferFrom` (or `safeTransferFrom`) can lead to loss of funds, because anyone can transfer tokens from the `from` address if an approval is made.")
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

    use crate::detect::{
        detector::IssueDetector, high::arbitrary_transfer_from::ArbitraryTransferFromDetector,
    };

    #[test]

    fn test_arbitrary_transfer_from_detector_by_loading_contract_directly() {
        let context = crate::detect::test_utils::load_solidity_source_unit(
            "../tests/contract-playground/src/ArbitraryTransferFrom.sol",
        );

        let mut detector = ArbitraryTransferFromDetector::default();
        let found = detector.detect(&context).unwrap();
        assert!(found);
        assert_eq!(detector.instances().len(), 2);
    }
}
