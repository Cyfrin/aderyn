use std::{collections::BTreeMap, error::Error};

use crate::{
    ast::ModifierInvocation,
    context::{
        browser::ContextBrowser,
        loader::{ASTNode, ContextLoader},
    },
    detect::detector::{Detector, IssueSeverity},
    visitor::ast_visitor::{ASTConstVisitor, Node},
};
use eyre::Result;

#[derive(Default)]
pub struct CentralizationRiskDetector {
    found_centralization_risks: Vec<Option<ASTNode>>,

    // Keys are source file name and line number
    found_instances: BTreeMap<(String, usize), String>,
}

impl ASTConstVisitor for CentralizationRiskDetector {
    fn visit_contract_definition(&mut self, node: &crate::ast::ContractDefinition) -> Result<bool> {
        // if the node's base_contracts.base_name.name contains any of the following values, add the node
        // to the found_centralization_risks vector:
        // [
        //  "Owned", "Ownable", "Ownable2Step", "AccessControl", "AccessControlCrossChain", "AccessControlEnumerable",
        //  "Auth", "RolesAuthority", "MultiRolesAuthority"
        // ]

        for base_contract in node.base_contracts.iter() {
            if base_contract.base_name.name == "Owned"
                || base_contract.base_name.name == "Ownable"
                || base_contract.base_name.name == "Ownable2Step"
                || base_contract.base_name.name == "AccessControl"
                || base_contract.base_name.name == "AccessControlCrossChain"
                || base_contract.base_name.name == "AccessControlEnumerable"
                || base_contract.base_name.name == "Auth"
                || base_contract.base_name.name == "RolesAuthority"
                || base_contract.base_name.name == "MultiRolesAuthority"
            {
                self.found_centralization_risks
                    .push(Some(ASTNode::ContractDefinition(node.clone())));
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
    fn detect(
        &mut self,
        loader: &ContextLoader,
        browser: &mut ContextBrowser,
    ) -> Result<bool, Box<dyn Error>> {
        for source_unit in loader.source_units.iter() {
            source_unit.accept(self)?;
        }
        for modifier_invocation in self
            .found_centralization_risks
            .clone()
            .into_iter()
            .flatten()
        {
            if let ASTNode::ModifierInvocation(modifier_invocation) = modifier_invocation {
                self.found_instances.insert(
                    browser.get_node_sort_key(&ASTNode::ModifierInvocation(
                        modifier_invocation.clone(),
                    )),
                    modifier_invocation.src.clone(),
                );
            }
        }
        for contract_definition in self
            .found_centralization_risks
            .clone()
            .into_iter()
            .flatten()
        {
            if let ASTNode::ContractDefinition(contract_definition) = contract_definition {
                self.found_instances.insert(
                    browser.get_node_sort_key(&ASTNode::ContractDefinition(
                        contract_definition.clone(),
                    )),
                    contract_definition.src.clone(),
                );
            }
        }

        Ok(!self.found_instances.is_empty())
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

    fn instances(&self) -> BTreeMap<(String, usize), String> {
        self.found_instances.clone()
    }
}

#[cfg(test)]
mod centralization_risk_detector_tests {
    use crate::{
        context::browser::ContextBrowser,
        detect::detector::{detector_test_helpers::load_contract, Detector},
    };

    use super::CentralizationRiskDetector;

    #[test]
    fn test_centralization_risk_detector() {
        let context_loader =
            load_contract("../tests/contract-playground/out/AdminContract.sol/AdminContract.json");
        let mut context_browser = ContextBrowser::default_from(&context_loader);
        context_browser.build_parallel();
        let mut detector = CentralizationRiskDetector::default();
        let found = detector
            .detect(&context_loader, &mut context_browser)
            .unwrap();
        // assert that the detector found a centralization risk
        assert!(found);
        // assert that the number of instances found is 2
        assert_eq!(detector.instances().len(), 3);
        // assert that the severity is medium
        assert_eq!(
            detector.severity(),
            crate::detect::detector::IssueSeverity::Medium
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
