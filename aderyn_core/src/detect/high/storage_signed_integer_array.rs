use std::collections::BTreeMap;
use std::error::Error;

use crate::ast::{
    ASTNode, Expression, Identifier, NodeID, TupleExpression, TypeDescriptions, UnaryOperation,
};

use crate::capture;
use crate::context::browser::GetImmediateParent;
use crate::detect::detector::IssueDetectorNamePool;
use crate::{
    context::workspace_context::WorkspaceContext,
    detect::detector::{IssueDetector, IssueSeverity},
};
use eyre::Result;
use lazy_regex::regex;

#[derive(Default)]
pub struct StorageSignedIntegerArrayDetector {
    // Keys are: [0] source file name, [1] line number, [2] character location of node.
    // Do not add items manually, use `capture!` to add nodes to this BTreeMap.
    found_instances: BTreeMap<(String, usize, String), NodeID>,
}

impl IssueDetector for StorageSignedIntegerArrayDetector {
    fn detect(&mut self, context: &WorkspaceContext) -> Result<bool, Box<dyn Error>> {
        // Search for a literal array with one negative value in it
        for tuple_expression in context
            .tuple_expressions()
            .into_iter()
            .filter(|tuple_expression| tuple_expression.is_inline_array)
        {
            // First, make sure it's being assigned to an array pointer to storage
            if !is_tuple_being_assigned_to_storage_array(tuple_expression, context) {
                continue;
            }

            // Now, make sure there is at least 1 negative value in the tuple array
            let negative_component_present = tuple_expression.components.iter().any(|c| {
                if let Some(Expression::UnaryOperation(UnaryOperation { operator, .. })) = c {
                    return operator == "-";
                }
                false
            });

            if negative_component_present {
                capture!(self, context, tuple_expression);
            }
        }

        Ok(!self.found_instances.is_empty())
    }

    fn severity(&self) -> IssueSeverity {
        IssueSeverity::High
    }

    fn title(&self) -> String {
        String::from("High Issue Title")
    }

    fn description(&self) -> String {
        String::from("Description of the high issue.")
    }

    fn instances(&self) -> BTreeMap<(String, usize, String), NodeID> {
        self.found_instances.clone()
    }

    fn name(&self) -> String {
        format!("high-issue-template")
    }
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
    use crate::detect::{
        detector::IssueDetector,
        high::storage_signed_integer_array::{
            StorageSignedIntegerArrayDetector, SIGNED_STORAGE_ARRAY_POINTER,
        },
    };

    #[test]
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
        assert_eq!(detector.title(), String::from("High Issue Title"));
        // assert the description is correct
        assert_eq!(
            detector.description(),
            String::from("Description of the high issue.")
        );
    }

    #[test]
    fn test_regular_expression_works() {
        // TARGET signed storage array references

        assert_eq!(
            SIGNED_STORAGE_ARRAY_POINTER.is_match("int256[3] storage ref"),
            true
        );
        assert_eq!(
            SIGNED_STORAGE_ARRAY_POINTER.is_match("int[1300] storage ref"),
            true
        );
        assert_eq!(
            SIGNED_STORAGE_ARRAY_POINTER.is_match("int8[] storage ref"),
            true
        );
        assert_eq!(
            SIGNED_STORAGE_ARRAY_POINTER.is_match("int[] storage ref"),
            true
        );
        assert_eq!(
            SIGNED_STORAGE_ARRAY_POINTER.is_match("uint256[3] storage ref"),
            false
        );
        assert_eq!(
            SIGNED_STORAGE_ARRAY_POINTER.is_match("uint[1300] storage ref"),
            false
        );
        assert_eq!(
            SIGNED_STORAGE_ARRAY_POINTER.is_match("uint8[] storage ref"),
            false
        );
        assert_eq!(
            SIGNED_STORAGE_ARRAY_POINTER.is_match("uint[] storage ref"),
            false
        );
    }
}
