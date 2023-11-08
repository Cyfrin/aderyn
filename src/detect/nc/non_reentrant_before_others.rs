use std::error::Error;

use crate::{
    context::{ast_node::ASTNode, loader::ContextLoader},
    detect::detector::{Detector, IssueSeverity},
};
use eyre::Result;

#[derive(Default)]
pub struct NonReentrantBeforeOthersDetector {
    found_non_reentrant_after_others: Vec<Option<ASTNode>>,
}

impl Detector for NonReentrantBeforeOthersDetector {
    fn detect(&mut self, loader: &ContextLoader) -> Result<bool, Box<dyn Error>> {
        let function_definitions = loader.get_function_definitions();
        for definition in function_definitions {
            if definition.modifiers.len() > 1 {
                for (index, modifier) in definition.modifiers.iter().enumerate() {
                    if modifier.modifier_name.name == "nonReentrant" && index != 0 {
                        self.found_non_reentrant_after_others
                            .push(Some(ASTNode::FunctionDefinition(definition.clone())));
                    }
                }
            }
        }
        Ok(!self.found_non_reentrant_after_others.is_empty())
    }

    fn title(&self) -> String {
        String::from("The `nonReentrant` `modifier` should occur before all other modifiers")
    }

    fn description(&self) -> String {
        String::from("This is a best-practice to protect against reentrancy in other modifiers")
    }

    fn severity(&self) -> IssueSeverity {
        IssueSeverity::NC
    }

    fn instances(&self) -> Vec<Option<ASTNode>> {
        self.found_non_reentrant_after_others.clone()
    }
}

#[cfg(test)]
mod non_reentrant_before_others_tests {
    use crate::detect::{
        detector::{detector_test_helpers::load_contract, Detector},
        nc::non_reentrant_before_others::NonReentrantBeforeOthersDetector,
    };

    #[test]
    fn test_non_reentrant_before_others() {
        let context_loader =
            load_contract("./tests/contract-playground/out/AdminContract.sol/AdminContract.json");
        let mut detector = NonReentrantBeforeOthersDetector::default();
        let found = detector.detect(&context_loader).unwrap();
        // assert that the detector found something
        assert!(found);
        // assert that the detector found the correct number
        assert_eq!(detector.instances().len(), 1);
        // assert that the detector returns the correct severity
        assert_eq!(
            detector.severity(),
            crate::detect::detector::IssueSeverity::NC
        );
        // assert that the detector returns the correct title
        assert_eq!(
            detector.title(),
            "The `nonReentrant` `modifier` should occur before all other modifiers"
        );
        // assert that the detector returns the correct description
        assert_eq!(
            detector.description(),
            "This is a best-practice to protect against reentrancy in other modifiers"
        );
    }
}
