use std::collections::BTreeMap;
use std::error::Error;
use std::str::FromStr;

use crate::ast::{
    ASTNode, Expression, Identifier, NodeID, TupleExpression, TypeDescriptions, UnaryOperation,
};

use crate::capture;
use crate::context::browser::{
    ExtractPragmaDirectives, ExtractTupleExpressions, GetImmediateParent,
};
use crate::detect::detector::IssueDetectorNamePool;
use crate::detect::helpers;
use crate::{
    context::workspace_context::WorkspaceContext,
    detect::detector::{IssueDetector, IssueSeverity},
};
use eyre::Result;
use lazy_regex::regex;
use semver::{Version, VersionReq};

#[derive(Default)]
pub struct StorageSignedIntegerArrayDetector {
    // Keys are: [0] source file name, [1] line number, [2] character location of node.
    // Do not add items manually, use `capture!` to add nodes to this BTreeMap.
    found_instances: BTreeMap<(String, usize, String), NodeID>,
}

impl IssueDetector for StorageSignedIntegerArrayDetector {
    fn detect(&mut self, context: &WorkspaceContext) -> Result<bool, Box<dyn Error>> {
        for source_unit in context.source_units() {
            let tuple_expressions = ExtractTupleExpressions::from(source_unit).extracted;
            let pragma_directives = ExtractPragmaDirectives::from(source_unit).extracted;

            if let Some(pragma_directive) = pragma_directives.first() {
                if let Ok(pragma_semver) = helpers::pragma_directive_to_semver(pragma_directive) {
                    if version_req_allows_below_0_5_10(&pragma_semver) {
                        // Search for a literal array with one negative value in it
                        for tuple_expression in tuple_expressions
                            .into_iter()
                            .filter(|tuple_expression| tuple_expression.is_inline_array)
                        {
                            // First, make sure it's being assigned to an array pointer to storage
                            if !is_tuple_being_assigned_to_storage_array(&tuple_expression, context)
                            {
                                continue;
                            }

                            // Now, make sure there is at least 1 negative value in the tuple array
                            let negative_component_present =
                                tuple_expression.components.iter().any(|c| {
                                    if let Some(Expression::UnaryOperation(UnaryOperation {
                                        operator,
                                        ..
                                    })) = c
                                    {
                                        return operator == "-";
                                    }
                                    false
                                });

                            if negative_component_present {
                                capture!(self, context, tuple_expression);
                            }
                        }
                    }
                }
            }
        }

        Ok(!self.found_instances.is_empty())
    }

    fn severity(&self) -> IssueSeverity {
        IssueSeverity::High
    }

    fn title(&self) -> String {
        String::from(
            "Compiler Bug: Signed array in storage detected for compiler version `<0.5.10`",
        )
    }

    fn description(&self) -> String {
        String::from("If you want to leverage signed arrays in storage by assigning a literal array with at least one negative \
            number, then you mus use solidity version 0.5.10 or above. This is because of a bug in older compilers.")
    }

    fn instances(&self) -> BTreeMap<(String, usize, String), NodeID> {
        self.found_instances.clone()
    }

    fn name(&self) -> String {
        IssueDetectorNamePool::SignedStorageArray.to_string()
    }
}

fn version_req_allows_below_0_5_10(version_req: &VersionReq) -> bool {
    // If it matches any 0.4.0 to 0.4.26, return true
    for i in 0..=26 {
        let version = Version::from_str(&format!("0.4.{}", i)).unwrap();
        if version_req.matches(&version) {
            return true;
        }
    }

    // If it matches any 0.5.0 to 0.5.9 return true
    for i in 0..=9 {
        let version = Version::from_str(&format!("0.5.{}", i)).unwrap();
        if version_req.matches(&version) {
            return true;
        }
    }

    // Else, return false
    false
}

// Build a regular expression to catch type names that correspond to pointers to storage arrays
static SIGNED_STORAGE_ARRAY_POINTER: &lazy_regex::Lazy<lazy_regex::Regex> =
    regex!(r"^int[0-9]*\[[0-9]*] storage ref$");

fn is_tuple_being_assigned_to_storage_array(
    tuple_expression: &TupleExpression,
    context: &WorkspaceContext,
) -> bool {
    if let Some(ASTNode::Assignment(assignment)) = tuple_expression.parent(context) {
        if let Expression::Identifier(Identifier {
            type_descriptions:
                TypeDescriptions {
                    type_string: Some(type_string),
                    ..
                },
            ..
        }) = assignment.left_hand_side.as_ref()
        {
            if SIGNED_STORAGE_ARRAY_POINTER.is_match(type_string) {
                return true;
            }
        }
    }
    false
}

#[cfg(test)]
mod storage_signed_array_detector {
    use serial_test::serial;

    use crate::detect::{
        detector::IssueDetector,
        high::storage_signed_integer_array::{
            StorageSignedIntegerArrayDetector, SIGNED_STORAGE_ARRAY_POINTER,
        },
    };

    #[test]
    #[serial]
    fn test_storage_signed_array() {
        let context = crate::detect::test_utils::load_solidity_source_unit(
            "../tests/contract-playground/src/CompilerBugStorageSignedIntegerArray.sol",
        );

        let mut detector = StorageSignedIntegerArrayDetector::default();
        let found = detector.detect(&context).unwrap();

        println!("{:?}", detector.instances());

        // assert that the detector found an issue
        assert!(found);
        // assert that the detector found the correct number of instances
        assert_eq!(detector.instances().len(), 1);
        // assert the severity is high
        assert_eq!(
            detector.severity(),
            crate::detect::detector::IssueSeverity::High
        );
        // assert the title is correct
        assert_eq!(
            detector.title(),
            String::from(
                "Compiler Bug: Signed array in storage detected for compiler version `<0.5.10`",
            )
        );
        // assert the description is correct
        assert_eq!(
            detector.description(),
            String::from("If you want to leverage signed arrays in storage by assigning a literal array with at least one negative \
                number, then you mus use solidity version 0.5.10 or above. This is because of a bug in older compilers.")
        );
    }

    #[test]
    fn test_regular_expression_works() {
        // TARGET signed storage array references

        assert!(SIGNED_STORAGE_ARRAY_POINTER.is_match("int256[3] storage ref"));
        assert!(SIGNED_STORAGE_ARRAY_POINTER.is_match("int[1300] storage ref"));
        assert!(SIGNED_STORAGE_ARRAY_POINTER.is_match("int8[] storage ref"));
        assert!(SIGNED_STORAGE_ARRAY_POINTER.is_match("int[] storage ref"));
        assert!(!SIGNED_STORAGE_ARRAY_POINTER.is_match("uint256[3] storage ref"));
        assert!(!SIGNED_STORAGE_ARRAY_POINTER.is_match("uint[1300] storage ref"));
        assert!(!SIGNED_STORAGE_ARRAY_POINTER.is_match("uint8[] storage ref"));
        assert!(!SIGNED_STORAGE_ARRAY_POINTER.is_match("uint[] storage ref"));
    }
}
