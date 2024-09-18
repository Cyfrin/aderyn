use std::{collections::BTreeMap, error::Error, str::FromStr};

use crate::{
    ast::NodeID,
    capture,
    context::workspace_context::WorkspaceContext,
    detect::{
        detector::{IssueDetector, IssueDetectorNamePool, IssueSeverity},
        helpers::pragma_directive_to_semver,
    },
};
use eyre::Result;
use semver::{Version, VersionReq};

#[derive(Default)]
pub struct MCOPYOpcodeDetector {
    // Keys are: [0] source file name, [1] line number, [2] character location of node.
    // Do not add items manually, use `capture!` to add nodes to this BTreeMap.
    found_instances: BTreeMap<(String, usize, String), NodeID>,
}

impl IssueDetector for MCOPYOpcodeDetector {
    fn detect(&mut self, context: &WorkspaceContext) -> Result<bool, Box<dyn Error>> {
        for pragma_directive in context.pragma_directives() {
            let req = pragma_directive_to_semver(pragma_directive)?;
            if version_req_allows_0_8_23_and_above(&req) {
                capture!(self, context, pragma_directive);
            }
        }
        Ok(!self.found_instances.is_empty())
    }

    fn severity(&self) -> IssueSeverity {
        IssueSeverity::Low
    }

    fn title(&self) -> String {
        String::from("PUSH0 is not supported by all chains")
    }

    fn description(&self) -> String {
        String::from("MCOPY opcode is introduced in Solidity 0.8.23 so it's very likely that not all chains support it yet. Consider using using older version of solidity.")
    }

    fn instances(&self) -> BTreeMap<(String, usize, String), NodeID> {
        self.found_instances.clone()
    }

    fn name(&self) -> String {
        format!("{}", IssueDetectorNamePool::MCOPYOpcode)
    }
}

fn version_req_allows_0_8_23_and_above(version_req: &VersionReq) -> bool {
    // If it matches any above 0.8.23 return true
    // TODO: Every time a solidity version is released, please increment the below counter by 1
    let current_highest_version = 27;
    for i in 23..=current_highest_version {
        let version = Version::from_str(&format!("0.8.{}", i)).unwrap();
        if version_req.matches(&version) {
            return true;
        }
    }

    // Else, return false
    false
}

#[cfg(test)]
mod mcopy_opcode_tests {
    use serial_test::serial;

    use crate::detect::detector::IssueDetector;

    #[test]
    #[serial]
    fn test_mcopy_opcode() {
        let context = crate::detect::test_utils::load_solidity_source_unit(
            "../tests/contract-playground/src/LatestSolidityVersionContract.sol",
        );

        let mut detector = super::MCOPYOpcodeDetector::default();
        let found = detector.detect(&context).unwrap();
        // assert that it found something
        assert!(found);
        // assert that the number of instances is correct
        assert_eq!(detector.instances().len(), 1);
        // assert that the severity is low
        assert_eq!(
            detector.severity(),
            crate::detect::detector::IssueSeverity::Low
        );
    }
}
