use std::error::Error;

use crate::{
    ast::{ModifierInvocation, SourceUnit},
    context::loader::{ASTNode, ContextLoader},
    detector::detector::{Detector, IssueSeverity},
    visitor::ast_visitor::{ASTConstVisitor, Node},
};
use eyre::Result;

#[derive(Default)]
pub struct CentralizationRiskDetector {
    pub found_centralization_risks: Vec<Option<ASTNode>>,
}

impl ASTConstVisitor for CentralizationRiskDetector {
    fn visit_source_unit(&mut self, node: &SourceUnit) -> Result<bool> {
        // if the node's exported_symbols HashMap contains a key with any of the following values, add the node
        // to the found_centralization_risks vector:
        // [
        //   "Owned", "Ownable", "Ownable2Step", "AccessControl", "AccessControlCrossChain", "AccessControlEnumerable",
        //   "Auth", "RolesAuthority", "MultiRolesAuthority"
        // ]
        if node.exported_symbols.is_some() {
            let exported_symbols = node.exported_symbols.as_ref().unwrap();
            let keys = exported_symbols.keys();
            for key in keys {
                if key == "Owned"
                    || key == "Ownable"
                    || key == "Ownable2Step"
                    || key == "AccessControl"
                    || key == "AccessControlCrossChain"
                    || key == "AccessControlEnumerable"
                    || key == "Auth"
                    || key == "RolesAuthority"
                    || key == "MultiRolesAuthority"
                {
                    self.found_centralization_risks
                        .push(Some(ASTNode::SourceUnit(node.clone())));
                }
            }
        }

        Ok(true)
    }

    fn visit_modifier_invocation(&mut self, node: &ModifierInvocation) -> Result<bool> {
        // If the node's modifier_name (IdentifierPath) is equal to "onlyOwner", "requiresAuth", or contains ["onlyRole"],
        // add the node to the found_centralization_risks vector
        if node.modifier_name.name == "onlyOwner"
            || node.modifier_name.name == "requiresAuth"
            || node.modifier_name.name.contains("onlyRole")
        {
            self.found_centralization_risks
                .push(Some(ASTNode::ModifierInvocation(node.clone())));
        }
        Ok(true)
    }
}

impl Detector for CentralizationRiskDetector {
    fn detect(&mut self, loader: &ContextLoader) -> Result<bool, Box<dyn Error>> {
        for source_unit in loader.get_source_units() {
            source_unit.accept(self)?;
        }

        Ok(self.found_centralization_risks.len() > 0)
    }

    fn severity(&self) -> IssueSeverity {
        IssueSeverity::Medium
    }

    fn title(&self) -> String {
        String::from("Centralization Risk for trusted owners")
    }

    fn description(&self) -> String {
        String::from("Contracts have owners with privileged rights to perform admin tasks and need to be trusted to not perform malicious updates or drain funds.")
    }

    fn instances(&self) -> Vec<Option<ASTNode>> {
        self.found_centralization_risks.clone()
    }
}

#[cfg(test)]
mod centralization_risk_detector_tests {
    use crate::detector::detector::{detector_test_helpers::load_contract, Detector};

    use super::CentralizationRiskDetector;

    #[test]
    fn test_centralization_risk_detector() {
        let context_loader =
            load_contract("./tests/contract-playground/out/AdminContract.sol/AdminContract.json");
        let mut detector = CentralizationRiskDetector::default();
        let found = detector.detect(&context_loader).unwrap();
        // assert that the detector found a centralization risk
        assert!(found);
        // assert that the number of instances found is 2
        assert_eq!(detector.instances().len(), 2);
        // assert that the severity is medium
        assert_eq!(
            detector.severity(),
            crate::detector::detector::IssueSeverity::Medium
        );
        // assert that the title is correct
        assert_eq!(
            detector.title(),
            String::from("Centralization Risk for trusted owners")
        );
        // assert that the description is correct
        assert_eq!(
            detector.description(),
            String::from(
                "Contracts have owners with privileged rights to perform admin tasks and need to be trusted to not perform malicious updates or drain funds."
            )
        );
    }
}
