use std::collections::BTreeMap;

use std::{convert::identity, error::Error};

use crate::ast::NodeID;

use crate::{
    capture,
    context::workspace_context::WorkspaceContext,
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
        for contract in context.contract_definitions() {
            // If a contract can accept eth, but doesn't allow for withdrawal capture it!
            if contract.can_accept_eth(context).is_some_and(identity)
                && !contract.allows_withdrawal_of_eth(context).is_some_and(identity)
            {
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
        ast::{ASTNode, ContractDefinition, StateMutability, Visibility},
        context::{
            browser::ExtractFunctionDefinitions,
            graph::{CallGraph, CallGraphDirection, CallGraphVisitor},
            workspace_context::WorkspaceContext,
        },
        detect::helpers,
    };

    impl ContractDefinition {
        pub fn can_accept_eth(&self, context: &WorkspaceContext) -> Option<bool> {
            let contracts = self.linearized_base_contracts.as_ref()?;
            for contract_id in contracts {
                let funcs =
                    ExtractFunctionDefinitions::from(context.nodes.get(contract_id)?).extracted;
                let num_payable_funcs = funcs
                    .into_iter()
                    .filter(|f| f.implemented && *f.state_mutability() == StateMutability::Payable)
                    .count();
                if num_payable_funcs > 0 {
                    return Some(true);
                }
            }
            Some(false)
        }

        pub fn allows_withdrawal_of_eth(&self, context: &WorkspaceContext) -> Option<bool> {
            /*
                For all the contracts in the hierarchy try and see if there is exists a public/external function that
                can be called which takes the execution flow in a path where there is possibility to send back eth away from
                the contract using the low level `call{value: XXX}` or `transfer` or `send`.
            */
            let contracts = self.linearized_base_contracts.as_ref()?;
            for contract_id in contracts {
                if let ASTNode::ContractDefinition(contract) = context.nodes.get(contract_id)? {
                    let funcs = contract
                        .function_definitions()
                        .into_iter()
                        .filter(|f| {
                            f.implemented
                                && (f.visibility == Visibility::Public
                                    || f.visibility == Visibility::External)
                        })
                        .map(|f| f.into())
                        .collect::<Vec<ASTNode>>();

                    let mut tracker = EthWithdrawalAllowerTracker::default();

                    let callgraph = CallGraph::new(
                        context,
                        funcs.iter().collect::<Vec<_>>().as_slice(),
                        CallGraphDirection::Inward,
                    )
                    .ok()?;

                    callgraph.accept(context, &mut tracker).ok()?;

                    if tracker.has_calls_that_sends_native_eth {
                        return Some(true);
                    }
                }
            }
            // At this point we have successfully gone through all the contracts in the inheritance
            // hierarchy but tracker has determined that none of them have have calls
            // that sends native eth Even if they are by some chance, they are not
            // reachable from external & public functions
            Some(false)
        }
    }

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

        // assert that the detector found an issue
        assert!(found);
        // assert that the detector found the correct number of instances
        assert_eq!(detector.instances().len(), 2);
    }
}
