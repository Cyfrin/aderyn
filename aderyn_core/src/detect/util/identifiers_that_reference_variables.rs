use std::collections::BTreeMap;
use std::error::Error;

use crate::{
    ast::NodeID,
    capture,
    context::workspace_context::{ASTNode, WorkspaceContext},
    detect::detector::{Detector, IssueSeverity},
};

#[derive(Default)]
pub struct IdentifiersThatReferenceVariableDetector {
    // Keys are source file name and line number
    found_instances: BTreeMap<(String, usize), NodeID>,
}

impl Detector for IdentifiersThatReferenceVariableDetector {
    fn detect(
        &mut self,
        context: &WorkspaceContext,
        _: &[NodeID],
        using: &[NodeID],
    ) -> std::prelude::v1::Result<bool, Box<dyn Error>> {
        if using.is_empty() {
            return Err("Error: using is empty".into());
        }
        if using.len() > 1 {
            return Err("Error: using is greater than 1".into());
        }

        let retrieved = context.nodes.get(&using[0]);
        if retrieved.is_none() {
            return Err("Error: retrieved is none".into());
        }
        let retrieved = retrieved.unwrap();
        if let ASTNode::VariableDeclaration(variable_declaration) = retrieved {
            context.identifiers.keys().for_each(|identifier| {
                if identifier.referenced_declaration == variable_declaration.id {
                    capture!(self, context, identifier);
                }
            });
        } else {
            return Err("Error: retrieved is not a variable declaration".into());
        }

        Ok(!self.found_instances.is_empty())
    }

    fn severity(&self) -> IssueSeverity {
        IssueSeverity::Utility
    }

    fn title(&self) -> String {
        String::from("Literals that reference a variable declaration")
    }

    fn description(&self) -> String {
        String::from("Get all literals in the context that reference a variable declaration")
    }

    fn name(&self) -> String {
        "IdentifiersThatReferenceVariableDetector".to_string()
    }

    fn instances(&self) -> BTreeMap<(String, usize), NodeID> {
        self.found_instances.clone()
    }
}

#[cfg(test)]
mod identifiers_that_reference_variables_detector_tests {
    use crate::detect::detector::{detector_test_helpers::load_contract, Detector};

    use super::IdentifiersThatReferenceVariableDetector;

    #[test]
    fn test_delegate_call_in_loop_detector() {
        let context = load_contract(
            "../tests/contract-playground/out/UniswapV2Swapper.sol/UniswapV2Swapper.json",
        );
        // from context, get the first item from variable_declarations where name is "amountIn"
        let variable_declaration = context
            .variable_declarations
            .keys()
            .find(|variable_declaration| variable_declaration.name == "amountIn")
            .unwrap();

        let mut detector = IdentifiersThatReferenceVariableDetector::default();
        let found = detector
            .detect(&context, &[], &[variable_declaration.id])
            .unwrap();
        // assert that the detector found
        assert!(found);
        // assert that the detector found the correct number of instances
        assert_eq!(detector.instances().len(), 4);
        // assert the severity is high
        assert_eq!(
            detector.severity(),
            crate::detect::detector::IssueSeverity::Utility
        );
        // assert the title is correct
        assert_eq!(
            detector.title(),
            String::from("Literals that reference a variable declaration")
        );
        // assert the description is correct
        assert_eq!(
            detector.description(),
            String::from("Get all literals in the context that reference a variable declaration")
        );
    }
}
