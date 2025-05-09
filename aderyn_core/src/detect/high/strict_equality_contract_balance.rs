use std::{collections::BTreeMap, error::Error};

use crate::ast::{Expression, NodeID};

use crate::{
    capture,
    context::workspace::WorkspaceContext,
    detect::detector::{IssueDetector, IssueDetectorNamePool, IssueSeverity},
};
use eyre::Result;

#[derive(Default)]
pub struct DangerousStrictEqualityOnBalanceDetector {
    // Keys are: [0] source file name, [1] line number, [2] character location of node.
    // Do not add items manually, use `capture!` to add nodes to this BTreeMap.
    found_instances: BTreeMap<(String, usize, String), NodeID>,
}

impl IssueDetector for DangerousStrictEqualityOnBalanceDetector {
    fn detect(&mut self, context: &WorkspaceContext) -> Result<bool, Box<dyn Error>> {
        // When you have found an instance of the issue,
        // use the following macro to add it to `found_instances`:
        //
        // capture!(self, context, item);

        for binary_operation in context
            .binary_operations()
            .into_iter()
            .filter(|&op| op.operator == "==" || op.operator == "!=")
        {
            for expr in [
                binary_operation.left_expression.as_ref(),
                binary_operation.right_expression.as_ref(),
            ] {
                if let Expression::MemberAccess(member_access) = expr {
                    if member_access.member_name == "balance"
                        && member_access.expression.as_ref().type_descriptions().is_some_and(
                            |type_desc| {
                                type_desc.type_string.as_ref().is_some_and(|type_string| {
                                    // For older solc versions when you say this.balance, "this" is
                                    // of type contract XXX
                                    type_string.starts_with("contract ")
                                    // In newers solidity versions, you say address(this).balance or payable(address(this)).balance
                                        || type_string == "address"
                                        || type_string == "address payable"
                                })
                            },
                        )
                    {
                        capture!(self, context, binary_operation);
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
        String::from("Dangerous strict equality checks on contract balances")
    }

    fn description(&self) -> String {
        String::from("A contract's balance can be forcibly manipulated by another selfdestructing contract. Therefore, it's recommended to use >, <, >= or <= instead of strict equality.")
    }

    fn instances(&self) -> BTreeMap<(String, usize, String), NodeID> {
        self.found_instances.clone()
    }

    fn name(&self) -> String {
        IssueDetectorNamePool::StrictEqualityContractBalance.to_string()
    }
}

#[cfg(test)]
mod strict_equality_contract_balance_tests {

    use crate::detect::{
        detector::IssueDetector,
        high::strict_equality_contract_balance::DangerousStrictEqualityOnBalanceDetector,
    };

    #[test]

    fn test_strict_equality_contract_balance1() {
        let context = crate::detect::test_utils::load_solidity_source_unit(
            "../tests/contract-playground/src/DangerousStrictEquality1.sol",
        );

        let mut detector = DangerousStrictEqualityOnBalanceDetector::default();
        let found = detector.detect(&context).unwrap();
        assert!(found);
        assert_eq!(detector.instances().len(), 1);
    }

    #[test]
    fn test_strict_equality_contract_balance2() {
        let context = crate::detect::test_utils::load_solidity_source_unit(
            "../tests/contract-playground/src/DangerousStrictEquality2.sol",
        );

        let mut detector = DangerousStrictEqualityOnBalanceDetector::default();
        let found = detector.detect(&context).unwrap();
        assert!(found);
        assert_eq!(detector.instances().len(), 2);
    }
}
