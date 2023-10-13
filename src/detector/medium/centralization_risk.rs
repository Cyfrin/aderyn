use std::error::Error;

use crate::{
    ast::{ModifierInvocation, SourceUnit},
    detector::detector::{Detector, IssueSeverity},
    loader::loader::{ASTNode, ContractLoader},
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
    fn detect(&mut self, loader: &ContractLoader) -> Result<bool, Box<dyn Error>> {
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
