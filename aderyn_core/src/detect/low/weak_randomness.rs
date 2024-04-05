use std::{collections::BTreeMap, error::Error};

use crate::{
    ast::{NodeID, TypeName},
    capture,
    context::{
        browser::{GetImmediateChildren, GetImmediateParent},
        workspace_context::{ASTNode, WorkspaceContext},
    },
    detect::detector::{IssueDetector, IssueDetectorNamePool, IssueSeverity},
};
use eyre::Result;

#[derive(Default)]
pub struct WeakRandomnessDetector {
    // Keys are source file name and line number
    found_instances: BTreeMap<(String, usize, String), NodeID>,
}

impl IssueDetector for WeakRandomnessDetector {
    fn detect(&mut self, context: &WorkspaceContext) -> Result<bool, Box<dyn Error>> {
        for identifier in context.identifiers() {
            if identifier.name == "keccak256" {
                if let Some(parent) = identifier.parent(context) {
                    if let Some(grandparent) = parent.parent(context) {
                        if let Some(uncles) = grandparent.children(context) {
                            for uncle in uncles {
                                if let ASTNode::ElementaryTypeNameExpression(e) = uncle {
                                    if let TypeName::ElementaryTypeName(type_name) = &e.type_name {
                                        if type_name.name == "uint" || type_name.name == "uint256" {
                                            capture!(self, context, identifier);
                                        }
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

    fn title(&self) -> String {
        String::from("Using `keccak256` produces weak randomness and is predicatable.")
    }

    fn description(&self) -> String {
        String::from("Consider using Chainlink VRF (Verifiable Random Function). It is a provably fair and verifiable random number generator (RNG) that enables smart contracts to access random values without compromising security or usability. For each request, Chainlink VRF generates one or more random values and cryptographic proof of how those values were determined. The proof is published and verified onchain before any consuming applications can use it. This process ensures that results cannot be tampered with or manipulated by any single entity including oracle operators, miners, users, or smart contract developers.")
    }

    fn severity(&self) -> IssueSeverity {
        IssueSeverity::Low
    }

    fn instances(&self) -> BTreeMap<(String, usize, String), NodeID> {
        self.found_instances.clone()
    }

    fn name(&self) -> String {
        format!("{}", IssueDetectorNamePool::WeakRandomness)
    }
}

#[cfg(test)]
mod weak_randomness {
    use crate::detect::{
        detector::{detector_test_helpers::load_contract, IssueDetector},
        low::WeakRandomnessDetector,
    };

    #[test]
    fn test_weak_randomness() {
        let context = load_contract(
            "../tests/contract-playground/out/WeakRandomness.sol/WeakRandomness.json",
        );

        let mut detector = WeakRandomnessDetector::default();
        let found = detector.detect(&context).unwrap();
        // assert that the detector found instances
        assert!(found);

        println!("{:?}", detector.instances());

        // assert that the detector found 2 instances of weak randomness
        assert!(detector.instances().len() == 2);
    }
}
