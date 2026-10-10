use std::{collections::BTreeMap, error::Error};

use crate::{
    ast::{
        ContractKind, ExtractAssignments, ExtractBinaryOperations, ExtractPragmaDirectives,
        ExtractUnaryOperations, NodeID, NodeType,
    },
    capture,
    context::{browser::GetClosestAncestorOfTypeX, workspace::WorkspaceContext},
    detect::{
        detector::{IssueDetector, IssueDetectorNamePool, IssueSeverity},
        helpers::pragma_directive_to_semver,
    },
};
use eyre::Result;
use semver::{Version, VersionReq};
use std::str::FromStr;

#[derive(Default)]
pub struct UnsafeMathPre08Detector {
    found_instances: BTreeMap<(String, usize, String), NodeID>,
}

fn version_req_allows_below_0_8_0(version_req: &VersionReq) -> bool {
    for minor in 4..=7 {
        let latest_patch = match minor {
            4 => 26,
            5 => 17,
            6 => 12,
            7 => 6,
            _ => unreachable!(),
        };

        for patch in 0..=latest_patch {
            let version = Version::from_str(&format!("0.{minor}.{patch}")).unwrap();
            if version_req.matches(&version) {
                return true;
            }
        }
    }

    false
}

impl IssueDetector for UnsafeMathPre08Detector {
    fn detect(&mut self, context: &WorkspaceContext) -> Result<bool, Box<dyn Error>> {
        for contract in context.contract_definitions() {
            if contract.kind == ContractKind::Interface {
                continue;
            }

            let Some(source_unit) =
                contract.closest_ancestor_of_type(context, NodeType::SourceUnit)
            else {
                continue;
            };

            // Check if the pragma allows compilation with < 0.8.0
            let pragmas = ExtractPragmaDirectives::from(source_unit).extracted;
            let mut allows_pre_08 = false;

            for pragma in pragmas {
                if let Ok(version_req) = pragma_directive_to_semver(&pragma) {
                    allows_pre_08 = version_req_allows_below_0_8_0(&version_req);
                    if allows_pre_08 {
                        break;
                    }
                }
            }

            if !allows_pre_08 {
                continue;
            }

            // Flag arithmetic operations
            let binary_ops = ExtractBinaryOperations::from(contract).extracted;
            for op in binary_ops {
                if ["+", "-", "*", "/", "%", "**"].contains(&op.operator.as_str()) {
                    capture!(self, context, op);
                }
            }

            let assignments = ExtractAssignments::from(contract).extracted;
            for op in assignments {
                if ["+=", "-=", "*=", "/=", "%="].contains(&op.operator.as_str()) {
                    capture!(self, context, op);
                }
            }

            let unary_ops = ExtractUnaryOperations::from(contract).extracted;
            for op in unary_ops {
                if ["++", "--"].contains(&op.operator.as_str()) {
                    capture!(self, context, op);
                }
            }
        }

        Ok(!self.found_instances.is_empty())
    }

    fn title(&self) -> String {
        String::from("Unsafe Mathematical Operation in pre-0.8.0 Solidity")
    }

    fn description(&self) -> String {
        String::from(
            "Contracts compiled with Solidity versions before 0.8.0 do not have built-in overflow/underflow protection. Consider using SafeMath.",
        )
    }

    fn severity(&self) -> IssueSeverity {
        IssueSeverity::Low
    }

    fn instances(&self) -> BTreeMap<(String, usize, String), NodeID> {
        self.found_instances.clone()
    }

    fn name(&self) -> String {
        format!("{}", IssueDetectorNamePool::UnsafeMathPre08)
    }
}

#[cfg(test)]
mod unsafe_math_pre_08_tests {
    use crate::detect::{
        detector::IssueDetector, low::unsafe_math_pre_08::UnsafeMathPre08Detector,
    };

    #[test]
    fn test_unsafe_math_pre_08_detector() {
        let context = crate::detect::test_utils::load_solidity_source_unit(
            "../tests/contract-playground/src/UnsafeMathPre08.sol",
        );

        let mut detector = UnsafeMathPre08Detector::default();
        let found = detector.detect(&context).unwrap();
        assert!(found);
        // add(y), sub(y), mul(y), safeAdd(a,b) have total: 3+3+2+1 = 9 operations flagged
        assert_eq!(detector.instances().len(), 9);
    }

    #[test]
    fn test_unsafe_math_pre_08_detector_on_0_4_26() {
        let context = crate::detect::test_utils::load_solidity_source_unit(
            "../tests/contract-playground/src/UnsafeMathPre08Exact0426.sol",
        );

        let mut detector = UnsafeMathPre08Detector::default();
        let found = detector.detect(&context).unwrap();
        assert!(found);
        assert_eq!(detector.instances().len(), 1);
    }

    #[test]
    fn test_unsafe_math_pre_08_detector_on_range() {
        let context = crate::detect::test_utils::load_solidity_source_unit(
            "../tests/contract-playground/src/UnsafeMathPre08Range.sol",
        );

        let mut detector = UnsafeMathPre08Detector::default();
        let found = detector.detect(&context).unwrap();
        assert!(found);
        assert_eq!(detector.instances().len(), 1);
    }

    #[test]
    fn test_unsafe_math_pre_08_detector_on_open_range() {
        let context = crate::detect::test_utils::load_solidity_source_unit(
            "../tests/contract-playground/src/UnsafeMathPre08OpenRange.sol",
        );

        let mut detector = UnsafeMathPre08Detector::default();
        let found = detector.detect(&context).unwrap();
        assert!(found);
        assert_eq!(detector.instances().len(), 1);
    }

    #[test]
    fn test_safe_math_post_08_detector() {
        let context = crate::detect::test_utils::load_solidity_source_unit(
            "../tests/contract-playground/src/SafeMathPost08.sol",
        );

        let mut detector = UnsafeMathPre08Detector::default();
        let found = detector.detect(&context).unwrap();
        assert!(!found);
        assert_eq!(detector.instances().len(), 0);
    }
}
