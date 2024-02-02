use crate::{context::workspace_context::ASTNode, detect::detector::ReusableDetector};

#[derive(Default)]
pub struct IdentifiersThatReferenceFunctionsDetector {
    // All the state variables, set at the beginning of the detect function
    found_instances: Vec<ASTNode>,
}

impl ReusableDetector for IdentifiersThatReferenceFunctionsDetector {
    fn detect(
        &mut self,
        context: &crate::context::workspace_context::WorkspaceContext,
        using: &[ASTNode],
        _: &[ASTNode],
    ) -> Result<&[ASTNode], Box<dyn std::error::Error>> {
        if using.is_empty() {
            return Err("Error: using is empty".into());
        }
        if using.len() > 1 {
            return Err("Error: using is greater than 1".into());
        }

        let retrieved = &using[0];
        if let ASTNode::FunctionDefinition(function_definition) = retrieved {
            context.identifiers.keys().for_each(|identifier| {
                if identifier.referenced_declaration == function_definition.id {
                    self.found_instances
                        .push(ASTNode::Identifier(identifier.clone()));
                }
            });
        } else {
            return Err("Error: retrieved is not a variable declaration".into());
        }

        Ok(&self.found_instances)
    }
}

#[cfg(test)]
mod identifiers_that_reference_functions_detector_tests {
    use crate::{
        context::workspace_context::ASTNode,
        detect::detector::{detector_test_helpers::load_contract, ReusableDetector},
    };

    use super::IdentifiersThatReferenceFunctionsDetector;

    #[test]
    fn test_identifiers_that_reference_functions_detector() {
        let context =
            load_contract("../tests/contract-playground/out/Counter.sol/Counter.0.8.21.json");
        // from context, get the first item from function_definitions where name is "amountIn"
        let function_definition = context
            .function_definitions
            .keys()
            .find(|function_definition| function_definition.name == "increment")
            .unwrap()
            .clone();

        let mut detector = IdentifiersThatReferenceFunctionsDetector::default();
        // create vec with function_definition as item 0
        let using = vec![ASTNode::FunctionDefinition(function_definition)];
        let found = detector.detect(&context, &using, &Vec::new()).unwrap();
        // assert that the detector found
        assert_eq!(found.len(), 1);
    }
}
