use std::{collections::BTreeMap, error::Error};

use crate::{
    ast::LiteralKind,
    capture,
    context::{
        browser::ExtractLiterals,
        loader::{ContextLoader},
    },
    detect::detector::{Detector, DetectorNamePool, IssueSeverity},
};
use eyre::Result;

#[derive(Default)]
pub struct ConstantsInsteadOfLiteralsDetector {
    // Keys are source file name and line number
    found_instances: BTreeMap<(String, usize), i64>,
}

impl Detector for ConstantsInsteadOfLiteralsDetector {
    fn detect(&mut self, loader: &ContextLoader) -> Result<bool, Box<dyn Error>> {
        // get all function definitions.
        // for each function definition, find all Literal types
        // if the literal type is either a Number, HexString or Address, then add it to the list of found literals
        for function_definition in loader.function_definitions.keys() {
            ExtractLiterals::from(function_definition)
                .extracted
                .iter()
                .for_each(|literal| {
                    if (literal.kind == LiteralKind::Number
                        && literal.value != Some(String::from("0")))
                        || literal.kind == LiteralKind::HexString
                        || literal.kind == LiteralKind::Address
                    {
                        capture!(self, loader, literal);
                    }
                });
        }

        Ok(!self.found_instances.is_empty())
    }

    fn title(&self) -> String {
        String::from("Constants should be defined and used instead of literals")
    }

    fn description(&self) -> String {
        String::from("")
    }

    fn severity(&self) -> IssueSeverity {
        IssueSeverity::NC
    }

    fn instances(&self) -> BTreeMap<(String, usize), i64> {
        self.found_instances.clone()
    }

    fn name(&self) -> String {
        format!("{}", DetectorNamePool::ConstantsInsteadOfLiterals)
    }
}

#[cfg(test)]
mod constants_instead_of_literals_tests {
    use crate::detect::detector::{detector_test_helpers::load_contract, Detector};

    use super::ConstantsInsteadOfLiteralsDetector;

    #[test]
    fn test_constants_instead_of_literals() {
        let context_loader =
            load_contract("../tests/contract-playground/out/Counter.sol/Counter.0.8.21.json");

        let mut detector = ConstantsInsteadOfLiteralsDetector::default();
        // assert that the detector finds the public function
        let found = detector.detect(&context_loader).unwrap();
        assert!(found);
        // assert that the detector finds the correct number of instances
        assert_eq!(detector.instances().len(), 1);
        // assert that the detector returns the correct severity
        assert_eq!(
            detector.severity(),
            crate::detect::detector::IssueSeverity::NC
        );
        // assert that the detector returns the correct title
        assert_eq!(
            detector.title(),
            String::from("Constants should be defined and used instead of literals")
        );
        // assert that the detector returns the correct description
        assert_eq!(detector.description(), String::from(""));
    }
}
