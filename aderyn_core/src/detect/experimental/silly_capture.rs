use std::{collections::BTreeMap, error::Error};

use crate::{
    ast::NodeID,
    capture_raw,
    context::workspace_context::{ASTNode, WorkspaceContext},
    detect::detector::{IssueDetector, IssueDetectorNamePool, IssueSeverity},
};
use eyre::Result;

#[derive(Default)]
pub struct SillyCaptureDetector {
    // Keys are source file name and line number
    found_instances: BTreeMap<(String, usize), NodeID>,
}

impl IssueDetector for SillyCaptureDetector {
    fn detect(&mut self, context: &WorkspaceContext) -> Result<bool, Box<dyn Error>> {
        let mut speculations: Vec<ASTNode> = vec![]; // I don't care what types I am holding they are all sus.

        for func in context.function_definitions() {
            if !func.name.contains("crement") {
                if func.parameters.parameters.is_empty() {
                    speculations.push(func.parameters.clone().into());
                }
            } else {
                if let Some(body) = &func.body {
                    speculations.push(body.clone().into());
                }
            }
        }

        for speculation in &speculations {
            capture_raw!(self, context, speculation);
        }

        Ok(!self.found_instances.is_empty())
    }

    fn title(&self) -> String {
        String::from("Silly capture")
    }

    fn description(&self) -> String {
        String::from("")
    }

    fn severity(&self) -> IssueSeverity {
        IssueSeverity::NC
    }

    fn instances(&self) -> BTreeMap<(String, usize), NodeID> {
        self.found_instances.clone()
    }

    fn name(&self) -> String {
        format!("{}", IssueDetectorNamePool::UselessPublicFunction)
    }
}

#[cfg(test)]
mod useless_public_function_tests {
    use crate::detect::detector::{detector_test_helpers::load_contract, IssueDetector};

    use super::SillyCaptureDetector;

    #[test]
    fn test_silly_capture_functions() {
        let context =
            load_contract("../tests/contract-playground/out/Counter.sol/Counter.0.8.21.json");

        let mut detector = SillyCaptureDetector::default();
        // assert that the detector finds the public function
        let found = detector.detect(&context).unwrap();
        assert!(found);

        println!("Instances: {:?}", detector.instances());
        assert!(detector.instances().len() == 4); // Line 11 16 21 28, Line 4 skipped because params is not empty
    }
}
