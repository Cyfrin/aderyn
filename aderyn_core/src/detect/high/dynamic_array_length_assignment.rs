use std::collections::BTreeMap;
use std::error::Error;

use crate::ast::NodeID;

use crate::capture;
use crate::detect::detector::IssueDetectorNamePool;
use crate::{
    context::workspace_context::WorkspaceContext,
    detect::detector::{IssueDetector, IssueSeverity},
};
use eyre::Result;

#[derive(Default)]
pub struct DynamicArrayLengthAssignmentDetector {
    // Keys are: [0] source file name, [1] line number, [2] character location of node.
    // Do not add items manually, use `capture!` to add nodes to this BTreeMap.
    found_instances: BTreeMap<(String, usize, String), NodeID>,
}

impl IssueDetector for DynamicArrayLengthAssignmentDetector {
    fn detect(&mut self, context: &WorkspaceContext) -> Result<bool, Box<dyn Error>> {
        for member_access in context
            .member_accesses()
            .into_iter()
            .filter(|member_acces| member_acces.l_value_requested)
        {
            let assignment_to = member_access.expression.type_descriptions();

            let is_being_assigned_on_dynamic_array = assignment_to.is_some_and(|assignment_to| {
                assignment_to
                    .type_string
                    .as_ref()
                    .is_some_and(|type_string| type_string.ends_with("[] storage ref"))
            });

            let is_being_assigned_to_length_property = member_access.member_name == "length";

            if is_being_assigned_on_dynamic_array && is_being_assigned_to_length_property {
                capture!(self, context, member_access);
            }
        }

        Ok(!self.found_instances.is_empty())
    }

    fn severity(&self) -> IssueSeverity {
        IssueSeverity::High
    }

    fn title(&self) -> String {
        String::from("Array length value has a direct assignment.")
    }

    fn description(&self) -> String {
        String::from(
            "If the length of a dynamic array (storage variable) directly assigned to, \
        it may allow access to other storage slots by tweaking it's value. This practice has \
        been depracated in newer Solidity versions",
        )
    }

    fn instances(&self) -> BTreeMap<(String, usize, String), NodeID> {
        self.found_instances.clone()
    }

    fn name(&self) -> String {
        IssueDetectorNamePool::DynamicArrayLengthAssignment.to_string()
    }
}

#[cfg(test)]
mod dynamic_array_length_assignment_tests {
    use serial_test::serial;

    use crate::detect::{
        detector::IssueDetector, high::DynamicArrayLengthAssignmentDetector,
        test_utils::load_solidity_source_unit,
    };

    #[test]
    #[serial]
    fn test_reused_contract_name_detector() {
        let context = load_solidity_source_unit(
            "../tests/contract-playground/src/DynamicArrayLengthAssignment.sol",
        );

        let mut detector = DynamicArrayLengthAssignmentDetector::default();
        let found = detector.detect(&context).unwrap();

        println!("{:#?}", detector.instances());

        // assert that the detector found an issue
        assert!(found);
        // assert that the detector found the correct number of instances
        assert_eq!(detector.instances().len(), 5);
        // assert the severity is high
        assert_eq!(
            detector.severity(),
            crate::detect::detector::IssueSeverity::High
        );
        // assert the title is correct
        assert_eq!(
            detector.title(),
            String::from("Array length value has a direct assignment.")
        );
        // assert the description is correct
        assert_eq!(
            detector.description(),
            String::from(
                "If the length of a dynamic array (storage variable) directly assigned to, \
            it may allow access to other storage slots by tweaking it's value. This practice has \
            been depracated in newer Solidity versions",
            )
        );
    }
}
