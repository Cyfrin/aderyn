use std::{collections::BTreeMap, error::Error};

use crate::{
    ast::{Expression, NodeID},
    capture,
    context::{
        browser::{
            ExtractMemberAccesses,
            GetImmediateParent, // Usa el re-export público aquí
        },
        workspace_context::{ASTNode, WorkspaceContext},
    },
    detect::detector::{IssueDetector, IssueDetectorNamePool, IssueSeverity},
};
use eyre::Result;

#[derive(Default)]
pub struct TxOriginAccesControl {
    found_instances: BTreeMap<(String, usize, String), NodeID>,
}

impl IssueDetector for TxOriginAccesControl {
    fn detect(&mut self, context: &WorkspaceContext) -> Result<bool, Box<dyn Error>> {
        // Filtra identificadores que coincidan con "require" o "IfStatement"
        let keywords = ["require", "IfStatement"];
        let relevant_identifiers: Vec<_> = context
            .identifiers()
            .into_iter()
            .filter(|&id| keywords.contains(&id.name.as_str()))
            .collect();

        for identifier in &relevant_identifiers {
            if identifier.name == "require" {
                if let Some(ASTNode::FunctionCall(fc)) = identifier.parent(context) {
                    let member_accesses = ExtractMemberAccesses::from(fc).extracted;
                    for member_access in &member_accesses {
                        if member_access.member_name == "origin" {
                            if let Expression::Identifier(identifier) =
                                member_access.expression.as_ref()
                            {
                                if identifier.name == "tx" {
                                    capture!(self, context, member_access);
                                }
                            }
                        }
                    }
                }
            }
        }

        for identifier in &relevant_identifiers {
            if identifier.name == "IfStatement" {
                if let Some(ASTNode::IfStatement(if_stmt)) = identifier.parent(context) {
                    // Evalúa la condición para encontrar `tx.origin`
                    let member_accesses = ExtractMemberAccesses::from(if_stmt).extracted;
                    for member_access in &member_accesses {
                        if member_access.member_name == "origin" {
                            if let Expression::Identifier(identifier) =
                                member_access.expression.as_ref()
                            {
                                if identifier.name == "tx" {
                                    capture!(self, context, member_access);
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
        IssueSeverity::Low
    }

    fn title(&self) -> String {
        String::from("Insecure tx.origin Authentication")
    }

    fn description(&self) -> String {
        String::from("Using tx.origin for authentication allows attackers to impersonate legitimate users, leading to unauthorized actions.")
    }

    fn instances(&self) -> BTreeMap<(String, usize, String), NodeID> {
        self.found_instances.clone()
    }

    fn name(&self) -> String {
        format!("{}", IssueDetectorNamePool::TxOriginAccesControl)
    }
}

#[cfg(test)]
mod tx_origin_detector_tests {
    use super::TxOriginAccesControl;
    use crate::detect::detector::{detector_test_helpers::load_contract, IssueDetector};

    #[test]
    fn test_tx_origin_detector() {
        let context = load_contract(
            "../tests/contract-playground/out/TxOriginAccessControl.sol/OriginControlledTransfer.json",
        );

        let mut detector = TxOriginAccesControl::default();
        let found = detector.detect(&context).unwrap();
        assert!(found);
        assert_eq!(detector.instances().len(), 2);
        assert_eq!(
            detector.severity(),
            crate::detect::detector::IssueSeverity::Low
        );
        // assert the title is correct
        assert_eq!(
            detector.title(),
            String::from("Insecure tx.origin Authentication")
        );
        // assert the description is correct
        assert_eq!(
            detector.description(),
            String::from("Using tx.origin for authentication allows attackers to impersonate legitimate users, leading to unauthorized actions.")
        );
    }
}
