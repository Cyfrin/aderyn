use std::{collections::BTreeMap, error::Error};

use crate::ast::NodeID;

use crate::{
    capture,
    context::workspace::WorkspaceContext,
    detect::detector::{IssueDetector, IssueDetectorNamePool, IssueSeverity},
};
use eyre::Result;

#[derive(Default)]
pub struct ExperimentalEncoderDetector {
    // Keys are: [0] source file name, [1] line number, [2] character location of node.
    // Do not add items manually, use `capture!` to add nodes to this BTreeMap.
    found_instances: BTreeMap<(String, usize, String), NodeID>,
}

impl IssueDetector for ExperimentalEncoderDetector {
    fn detect(&mut self, context: &WorkspaceContext) -> Result<bool, Box<dyn Error>> {
        for pragma_directive in context.pragma_directives() {
            for literal in &pragma_directive.literals {
                if literal == "experimental" {
                    capture!(self, context, pragma_directive);
                }
            }
        }

        Ok(!self.found_instances.is_empty())
    }

    fn severity(&self) -> IssueSeverity {
        IssueSeverity::High
    }

    fn title(&self) -> String {
        String::from("Experimental ABI Encoder")
    }

    fn description(&self) -> String {
        String::from("Experimental encoders should not be used in production. There are multiple known compiler bugs that are caused by the experimental encoder. Upgrade your solidity version to remove the need for experimental features.")
    }

    fn instances(&self) -> BTreeMap<(String, usize, String), NodeID> {
        self.found_instances.clone()
    }

    fn name(&self) -> String {
        IssueDetectorNamePool::ExperimentalEncoder.to_string()
    }
}

#[cfg(test)]
mod storage_array_encode_compiler_bug_detector_tests {

    use crate::detect::{
        detector::IssueDetector, high::experimental_encoder::ExperimentalEncoderDetector,
    };

    #[test]

    fn test_storage_array_encode_compiler_bug_detector() {
        let context = crate::detect::test_utils::load_solidity_source_unit(
            "../tests/contract-playground/src/ExperimentalEncoder.sol",
        );

        let mut detector = ExperimentalEncoderDetector::default();
        let found = detector.detect(&context).unwrap();
        assert!(found);
        assert_eq!(detector.instances().len(), 1);
    }
}
