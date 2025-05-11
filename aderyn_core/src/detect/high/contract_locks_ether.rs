use std::collections::BTreeMap;

use std::error::Error;

use crate::ast::NodeID;

use crate::{
    capture,
    context::workspace::WorkspaceContext,
    detect::detector::{IssueDetector, IssueDetectorNamePool, IssueSeverity},
};

use eyre::Result;

#[derive(Default)]
pub struct ContractLocksEtherDetector {
    // Keys are: [0] source file name, [1] line number, [2] character location of node.
    // Do not add items manually, use `capture!` to add nodes to this BTreeMap.
    found_instances: BTreeMap<(String, usize, String), NodeID>,
}

impl IssueDetector for ContractLocksEtherDetector {
    fn detect(&mut self, context: &WorkspaceContext) -> Result<bool, Box<dyn Error>> {
        for contract in context.deployable_contracts() {
            let Some(accepts_eth) = contract.can_accept_eth(context) else {
                continue;
            };
            let Some(allows_withdraw) = contract.allows_withdrawal_of_eth(context) else {
                continue;
            };
            if accepts_eth && !allows_withdraw {
                capture!(self, context, contract);
            }
        }
        Ok(!self.found_instances.is_empty())
    }

    fn severity(&self) -> IssueSeverity {
        IssueSeverity::High
    }

    fn title(&self) -> String {
        String::from("Contract locks Ether without a withdraw function")
    }

    fn description(&self) -> String {
        String::from(
            "It appears that the contract includes a payable function to accept Ether but lacks a corresponding function to withdraw it, \
            which leads to the Ether being locked in the contract. To resolve this issue, please implement a public or external function \
            that allows for the withdrawal of Ether from the contract."
        )
    }

    fn instances(&self) -> BTreeMap<(String, usize, String), NodeID> {
        self.found_instances.clone()
    }

    fn name(&self) -> String {
        IssueDetectorNamePool::ContractLocksEther.to_string()
    }
}

/// Handles tasks related to contract level analysis for eth
mod contract_eth_helper {
    use crate::{
        ast::{ASTNode, ContractDefinition, StateMutability},
        context::{
            graph::{CallGraphConsumer, CallGraphDirection, CallGraphVisitor},
            workspace::WorkspaceContext,
        },
        detect::helpers,
    };

    #[derive(Default)]
    struct EthWithdrawalAllowerTracker {
        has_calls_that_sends_native_eth: bool,
    }

    impl CallGraphVisitor for EthWithdrawalAllowerTracker {
        fn visit_any(&mut self, ast_node: &ASTNode) -> eyre::Result<()> {
            if !self.has_calls_that_sends_native_eth
                && helpers::has_calls_that_sends_native_eth(ast_node)
            {
                self.has_calls_that_sends_native_eth = true;
            }
            Ok(())
        }
    }

    impl ContractDefinition {
        pub(super) fn can_accept_eth(&self, context: &WorkspaceContext) -> Option<bool> {
            for func in self.entrypoint_functions(context)? {
                if *func.state_mutability() == StateMutability::Payable {
                    return Some(true);
                }
            }
            Some(false)
        }

        pub(super) fn allows_withdrawal_of_eth(&self, context: &WorkspaceContext) -> Option<bool> {
            for func in self.entrypoint_functions(context)? {
                let callgraphs =
                    CallGraphConsumer::get(context, &[&func.into()], CallGraphDirection::Inward)
                        .ok()?;
                for callgraph in callgraphs {
                    let mut tracker = EthWithdrawalAllowerTracker::default();
                    callgraph.accept(context, &mut tracker).ok()?;

                    if tracker.has_calls_that_sends_native_eth {
                        return Some(true);
                    }
                }
            }
            Some(false)
        }
    }
}

#[cfg(test)]
mod contract_locks_ether_detector_tests {

    use crate::detect::{
        detector::IssueDetector, high::contract_locks_ether::ContractLocksEtherDetector,
    };

    #[test]

    fn test_contract_locks_ether() {
        let context = crate::detect::test_utils::load_solidity_source_unit(
            "../tests/contract-playground/src/ContractLocksEther.sol",
        );

        let mut detector = ContractLocksEtherDetector::default();
        let found = detector.detect(&context).unwrap();

        assert!(found);
        assert_eq!(detector.instances().len(), 2);
    }
}
