use crate::{context::workspace_context::ASTNode, detect::detector::ReusableDetector};

#[derive(Default)]
pub struct IdentifiersThatReferenceVariablesDetector {
    // All the state variables, set at the beginning of the detect function
    found_instances: Vec<ASTNode>,
}

impl ReusableDetector for IdentifiersThatReferenceVariablesDetector {
    fn detect(
        &mut self,
        context: &crate::context::workspace_context::WorkspaceContext,
        using: &Vec<ASTNode>,
        _: &Vec<ASTNode>,
    ) -> Result<bool, Box<dyn std::error::Error>> {
        if using.is_empty() {
            return Err("Error: using is empty".into());
        }
        if using.len() > 1 {
            return Err("Error: using is greater than 1".into());
        }

        let retrieved = &using[0];
        if let ASTNode::VariableDeclaration(variable_declaration) = retrieved {
            context.identifiers.keys().for_each(|identifier| {
                if identifier.referenced_declaration == variable_declaration.id {
                    self.found_instances
                        .push(ASTNode::Identifier(identifier.clone()));
                }
            });
        } else {
            return Err("Error: retrieved is not a variable declaration".into());
        }

        Ok(!self.found_instances.is_empty())
    }

    fn instances(&self) -> Vec<&ASTNode> {
        self.found_instances.iter().collect()
    }
}

#[cfg(test)]
mod identifiers_that_reference_variables_detector_tests {
    use crate::{
        context::workspace_context::ASTNode,
        detect::detector::{detector_test_helpers::load_contract, ReusableDetector},
    };

    use super::IdentifiersThatReferenceVariablesDetector;

    #[test]
    fn test_identifiers_that_reference_variables_detector() {
        let context = load_contract(
            "../tests/contract-playground/out/UniswapV2Swapper.sol/UniswapV2Swapper.json",
        );
        // from context, get the first item from variable_declarations where name is "amountIn"
        let variable_declaration = context
            .variable_declarations
            .keys()
            .find(|variable_declaration| variable_declaration.name == "amountIn")
            .unwrap()
            .clone();

        let mut detector = IdentifiersThatReferenceVariablesDetector::default();
        // create vec with variable_declaration as item 0
        let using = vec![ASTNode::VariableDeclaration(variable_declaration)];
        let found = detector.detect(&context, &using, &Vec::new()).unwrap();
        // assert that the detector found
        assert!(found);
        // assert that the detector found the correct number of instances
        assert_eq!(detector.instances().len(), 4);
    }
}
