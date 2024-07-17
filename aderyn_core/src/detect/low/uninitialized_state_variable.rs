use std::collections::{BTreeMap, HashSet};
use std::error::Error;

use crate::ast::{Expression, NodeID};

use crate::capture;
use crate::detect::detector::IssueDetectorNamePool;
use crate::{
    context::workspace_context::WorkspaceContext,
    detect::detector::{IssueDetector, IssueSeverity},
};
use eyre::Result;

#[derive(Default)]
pub struct UninitializedStateVariableDetector {
    // Keys are: [0] source file name, [1] line number, [2] character location of node.
    // Do not add items manually, use `capture!` to add nodes to this BTreeMap.
    found_instances: BTreeMap<(String, usize, String), NodeID>,
}

impl IssueDetector for UninitializedStateVariableDetector {
    fn detect(&mut self, context: &WorkspaceContext) -> Result<bool, Box<dyn Error>> {
        /*
         * Plan (Maybe it can be improved)
         *  - Gather all the storage variables (VariableDeclarations)
         *  - Fitler out / Remove the ones where `value` property is not `None`
         *  - Fitler out / Remove the ones that are arrays, mappings and structs
         *  - Now, we're left with state variables that are not initialized at the same
         *    line where they are declared.
         *  - Gather all the `Assignments` and collect all the `referencedDeclarations` on
         *    `Identifier` expressions when they appear on LHS of the assginments
         *  - Remove the above ids from the initial storage variables list
         *  - Now we're left with storage variables that have never been initialized
         */

        let mut state_variable_ids = HashSet::new();

        for var_decl in context
            .variable_declarations()
            .into_iter()
            .filter(|s| s.state_variable && s.value.is_none())
            .filter(|s| {
                s.type_descriptions
                    .type_string
                    .as_ref()
                    .is_some_and(|type_string| !type_string.starts_with("mapping"))
            })
            .filter(|s| {
                s.type_descriptions
                    .type_string
                    .as_ref()
                    .is_some_and(|type_string| !type_string.ends_with("[]"))
            })
            .filter(|s| {
                s.type_descriptions
                    .type_string
                    .as_ref()
                    .is_some_and(|type_string| !type_string.starts_with("struct"))
            })
        {
            state_variable_ids.insert(var_decl.id);
        }

        for assignment in context.assignments() {
            if let Expression::Identifier(identifier) = assignment.left_hand_side.as_ref() {
                if let Some(refers_to) = identifier.referenced_declaration {
                    let _ = state_variable_ids.remove(&refers_to);
                }
            }
        }

        for id in state_variable_ids {
            context
                .nodes
                .get(&id)
                .inspect(|&x| capture!(self, context, x));
        }

        Ok(!self.found_instances.is_empty())
    }

    fn severity(&self) -> IssueSeverity {
        IssueSeverity::Low
    }

    fn title(&self) -> String {
        String::from("Uninitialized State Variables")
    }

    fn description(&self) -> String {
        String::from(
            "Solidity does initialize variables by default when you declare them, however it's good practice \
            to explicitly declare an initial value. For example, if you transfer money to an address we must make sure \
            that the address has been initialized."
        )
    }

    fn instances(&self) -> BTreeMap<(String, usize, String), NodeID> {
        self.found_instances.clone()
    }

    fn name(&self) -> String {
        IssueDetectorNamePool::UninitializedStateVariable.to_string()
    }
}

#[cfg(test)]
mod uninitialized_state_variable_tests {
    use crate::detect::{
        detector::IssueDetector,
        low::uninitialized_state_variable::UninitializedStateVariableDetector,
    };

    #[test]
    fn test_uninitialized_state_variables() {
        let context: crate::context::workspace_context::WorkspaceContext =
            crate::detect::test_utils::load_solidity_source_unit(
                "../tests/contract-playground/src/UninitializedStateVariable.sol",
            );

        let mut detector = UninitializedStateVariableDetector::default();
        let found = detector.detect(&context).unwrap();

        println!("{:?}", detector.instances());

        // assert that the detector found an issue
        assert!(found);
        // assert that the detector found the correct number of instances
        assert_eq!(detector.instances().len(), 2);
        // assert the severity is high
        assert_eq!(
            detector.severity(),
            crate::detect::detector::IssueSeverity::Low
        );
        // assert the title is correct
        assert_eq!(
            detector.title(),
            String::from("Uninitialized State Variables")
        );
        // assert the description is correct
        assert_eq!(
            detector.description(),
            String::from(
                "Solidity does initialize variables by default when you declare them, however it's good practice \
                to explicitly declare an initial value. For example, if you transfer money to an address we must make sure \
                that the address has been initialized."
            )
        );
    }
}
