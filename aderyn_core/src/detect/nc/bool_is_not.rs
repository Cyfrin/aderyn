use std::{collections::BTreeMap, error::Error};

use crate::{
    ast::{NodeID, TypeName},
    capture,
    context::workspace_context::WorkspaceContext,
    detect::detector::{IssueDetector, IssueDetectorNamePool, IssueSeverity},
};
use eyre::Result;

#[derive(Default)]
pub struct BooleanNameIsNotDoesNotDetector {
    // Keys are source file name and line number
    found_instances: BTreeMap<(String, usize, String), NodeID>,
}

impl IssueDetector for BooleanNameIsNotDoesNotDetector {
    fn detect(&mut self, context: &WorkspaceContext) -> Result<bool, Box<dyn Error>> {
        for variable_declaration in context.variable_declarations() {
            // println!(
            //     "{:#?} {:?}",
            //     variable_declaration.type_name, variable_declaration.name
            // );
            let var_name_lowercase = variable_declaration.name.to_lowercase();

            if let Some(TypeName::ElementaryTypeName(e)) = &variable_declaration.type_name {
                if e.name == "bool"
                    && (var_name_lowercase.starts_with("isnot")
                        || var_name_lowercase.starts_with("doesnot"))
                {
                    capture!(self, context, variable_declaration);
                }
            }
        }
        Ok(!self.found_instances.is_empty())
    }

    fn title(&self) -> String {
        String::from("Boolean variable name should not contain negatives.")
    }

    fn description(&self) -> String {
        String::from("In order to avoid confusion it's better to name variables with `isXX` and `doesXX` rather than `isNotXXX` and `doesNotXXX`.")
    }

    fn severity(&self) -> IssueSeverity {
        IssueSeverity::NC
    }

    fn instances(&self) -> BTreeMap<(String, usize, String), NodeID> {
        self.found_instances.clone()
    }

    fn name(&self) -> String {
        format!("{}", IssueDetectorNamePool::BooleanIsNotDoesNot)
    }
}

#[cfg(test)]
mod boolean_should_not_start_with_isnot {
    use crate::detect::{
        detector::{detector_test_helpers::load_contract, IssueDetector},
        nc::BooleanNameIsNotDoesNotDetector,
    };

    #[test]
    fn test_boolean_should_not_start_with_isnot() {
        let context =
            load_contract("../tests/contract-playground/out/BooleanIsNot.sol/BooleansIsNot.json");

        let mut detector = BooleanNameIsNotDoesNotDetector::default();
        let found = detector.detect(&context).unwrap();
        // assert that the detector found something
        assert!(found);
        // assert that the detector found the correct number
        assert_eq!(detector.instances().len(), 2);
        // assert that the detector returns the correct severity
        assert_eq!(
            detector.severity(),
            crate::detect::detector::IssueSeverity::NC
        );
    }
}
