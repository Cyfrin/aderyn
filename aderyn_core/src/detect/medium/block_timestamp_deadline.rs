use std::{collections::BTreeMap, error::Error};

use crate::{
    ast::{Expression, FunctionCallKind},
    context::{
        browser::ContextBrowser,
        loader::{ASTNode, ContextLoader},
    },
    detect::detector::{Detector, IssueSeverity},
};
use eyre::Result;

#[derive(Default)]
pub struct BlockTimestampDeadlineDetector {
    // Keys are source file name and line number
    found_instances: BTreeMap<(String, usize), String>,
}

impl Detector for BlockTimestampDeadlineDetector {
    fn detect(
        &mut self,
        loader: &ContextLoader,
        browser: &mut ContextBrowser,
    ) -> Result<bool, Box<dyn Error>> {
        for call in loader.function_calls.keys() {
            // Uniswap V2 - Function Calls
            // For each FunctionCall, if the Expression is a MemberAccess that is named any of the following:
            // [
            //  swapExactTokensForTokens, swapTokensForExactTokens, swapExactETHForTokens, swapTokensForExactETH,
            //  swapExactTokensForETH, swapETHForExactTokens, swapExactTokensForTokensSupportingFeeOnTransferTokens,
            //  swapExactETHForTokensSupportingFeeOnTransferTokens, swapExactTokensForETHSupportingFeeOnTransferTokens
            // ]
            // If the last FunctionCall argument is a MemberAccess identifier with member_name "timestamp",
            // and the MemberAccess expression.name is "block", add the node to the found_block_timestamp_deadlines vector.
            if let Expression::MemberAccess(ref member_access) = *call.expression {
                if member_access.member_name == "swapExactTokensForTokens"
                    || member_access.member_name == "swapTokensForExactTokens"
                    || member_access.member_name == "swapExactETHForTokens"
                    || member_access.member_name == "swapTokensForExactETH"
                    || member_access.member_name == "swapExactTokensForETH"
                    || member_access.member_name == "swapETHForExactTokens"
                    || member_access.member_name
                        == "swapExactTokensForTokensSupportingFeeOnTransferTokens"
                    || member_access.member_name
                        == "swapExactETHForTokensSupportingFeeOnTransferTokens"
                    || member_access.member_name
                        == "swapExactTokensForETHSupportingFeeOnTransferTokens"
                {
                    if let Expression::MemberAccess(ref member_access) =
                        call.arguments.last().unwrap()
                    {
                        if member_access.member_name == "timestamp" {
                            if let Expression::Identifier(ref identifier) =
                                *member_access.expression
                            {
                                if identifier.name == "block" {
                                    self.found_instances.insert(
                                        loader.get_node_sort_key(&ASTNode::FunctionCall(
                                            call.clone(),
                                        )),
                                        call.src.clone(),
                                    );
                                }
                            }
                        }
                    }
                }
            }
            // Uniswap V3 - Function Calls
            // For each FunctionCall, if it is of kind StructConstructorCall, where the call's Expression has a name of any of the following:
            // [
            //  ExactInputSingleParams, ExactInputParams, ExactOutputSingleParams, ExactOutputParams
            // ]
            // If any of the call's arguments is a MemberAccess identifier with member_name "timestamp",
            // and the MemberAccess expression.name is "block", add the node to the found_block_timestamp_deadlines vector.
            if call.kind == FunctionCallKind::StructConstructorCall {
                if let Expression::Identifier(ref identifier) = *call.expression {
                    if identifier.name == "ExactInputSingleParams"
                        || identifier.name == "ExactInputParams"
                        || identifier.name == "ExactOutputSingleParams"
                        || identifier.name == "ExactOutputParams"
                    {
                        for argument in call.arguments.iter() {
                            if let Expression::MemberAccess(ref member_access) = *argument {
                                if member_access.member_name == "timestamp" {
                                    if let Expression::Identifier(ref identifier) =
                                        *member_access.expression
                                    {
                                        if identifier.name == "block" {
                                            self.found_instances.insert(
                                                loader.get_node_sort_key(&ASTNode::FunctionCall(
                                                    call.clone(),
                                                )),
                                                call.src.clone(),
                                            );
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }

        // TODO: Uniswap V3 - Struct definitions
        Ok(!self.found_instances.is_empty())
    }

    fn severity(&self) -> IssueSeverity {
        IssueSeverity::Medium
    }

    fn title(&self) -> String {
        "Using `block.timestamp` for swap deadline offers no protection".to_string()
    }

    fn description(&self) -> String {
        "In the PoS model, proposers know well in advance if they will propose one or consecutive blocks ahead of time. In such a scenario, a malicious validator can hold back the transaction and execute it at a more favourable block number.\
        Consider allowing function caller to specify swap deadline input parameter.".to_string()
    }

    fn instances(&self) -> BTreeMap<(String, usize), String> {
        self.found_instances.clone()
    }
}

#[cfg(test)]
mod block_timestamp_deadline_detector_tests {
    use crate::{
        context::browser::ContextBrowser,
        detect::{
            detector::{detector_test_helpers::load_contract, Detector},
            medium::block_timestamp_deadline::BlockTimestampDeadlineDetector,
        },
    };

    #[test]
    fn test_block_timestamp_deadline_uniswap_v2_detector() {
        let context_loader = load_contract(
            "../tests/contract-playground/out/UniswapV2Swapper.sol/UniswapV2Swapper.json",
        );
        let mut context_browser = ContextBrowser::default_from(&context_loader);
        context_browser.build_parallel();
        let mut detector = BlockTimestampDeadlineDetector::default();
        let found = detector
            .detect(&context_loader, &mut context_browser)
            .unwrap();
        // assert that the detector found
        assert!(found);
        // assert that the number of instances found is correct
        assert_eq!(detector.instances().len(), 9);
        // assert that the severity is medium
        assert_eq!(
            detector.severity(),
            crate::detect::detector::IssueSeverity::Medium
        );
        // assert that the title is correct
        assert_eq!(
            detector.title(),
            String::from("Using `block.timestamp` for swap deadline offers no protection")
        );
        // assert that the description is correct
        assert_eq!(
            detector.description(),
            String::from(
                "In the PoS model, proposers know well in advance if they will propose one or consecutive blocks ahead of time. In such a scenario, a malicious validator can hold back the transaction and execute it at a more favourable block number.\
        Consider allowing function caller to specify swap deadline input parameter."
            )
        );
    }

    #[test]
    fn test_block_timestamp_deadline_uniswap_v3_detector() {
        let context_loader = load_contract(
            "../tests/contract-playground/out/UniswapV3Swapper.sol/UniswapV3Swapper.json",
        );
        let mut context_browser = ContextBrowser::default_from(&context_loader);
        context_browser.build_parallel();
        let mut detector = BlockTimestampDeadlineDetector::default();
        let found = detector
            .detect(&context_loader, &mut context_browser)
            .unwrap();
        // assert that the detector found
        assert!(found);
        // assert that the number of instances found is correct
        assert_eq!(detector.instances().len(), 8);
        // assert that the severity is medium
        assert_eq!(
            detector.severity(),
            crate::detect::detector::IssueSeverity::Medium
        );
        // assert that the title is correct
        assert_eq!(
            detector.title(),
            String::from("Using `block.timestamp` for swap deadline offers no protection")
        );
        // assert that the description is correct
        assert_eq!(
            detector.description(),
            String::from(
                "In the PoS model, proposers know well in advance if they will propose one or consecutive blocks ahead of time. In such a scenario, a malicious validator can hold back the transaction and execute it at a more favourable block number.\
        Consider allowing function caller to specify swap deadline input parameter."
            )
        );
    }
}
