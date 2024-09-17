use std::collections::{BTreeMap, HashSet};
use std::error::Error;

use crate::ast::{FunctionKind, Mutability, NodeID};

use crate::capture;
use crate::context::browser::ApproximateStorageChangeFinder;
use crate::detect::detector::IssueDetectorNamePool;
use crate::detect::helpers;
use crate::{
    context::workspace_context::WorkspaceContext,
    detect::detector::{IssueDetector, IssueSeverity},
};

#[derive(Default)]
pub struct StateVariableCouldBeImmutableDetector {
    // Keys are: [0] source file name, [1] line number, [2] character location of node.
    // Do not add items manually, use `capture!` to add nodes to this BTreeMap.
    found_instances: BTreeMap<(String, usize, String), NodeID>,
}

impl IssueDetector for StateVariableCouldBeImmutableDetector {
    fn detect(&mut self, context: &WorkspaceContext) -> Result<bool, Box<dyn Error>> {
        // PLAN
        // 1. Collect all state variables that are not marked constant or immutable and are also
        // not structs/mappings/contracts (collection A)
        //
        // 2. Investigate every non constructor function and collect all the state variables that
        // could change (collection B)
        //
        // 3. Investigate every constructor function and collect all the state variables that could
        // change (Collection C)
        //
        // 4. Let collection R1 = collection C - collection B
        // This represent subset of state variables that only change in the constructor
        //
        // 5. Let collection  R2 = collection A intersection R1
        // This is the final result

        let mut collection_a = Vec::new();

        for variable in context.variable_declarations() {
            if variable.mutability() == Some(&Mutability::Immutable) {
                continue;
            }

            // Do not report it if it's a struct / mapping
            if variable
                .type_descriptions
                .type_string
                .as_ref()
                .is_some_and(|type_string| {
                    type_string.starts_with("mapping") || type_string.starts_with("struct")
                })
            {
                continue;
            }

            if variable.overrides.is_some() {
                continue;
            }

            if variable.state_variable && !variable.constant {
                collection_a.push(variable);
            }
        }

        let mut state_var_changed_from_non_constructors =
            ApproximateStorageChangeFinder::default(context);
        for func in helpers::get_implemented_external_and_public_functions(context) {
            if *func.kind() == FunctionKind::Constructor {
                continue;
            }
            if let Some(changes) = func.state_variable_changes(context) {
                let new_changes = state_var_changed_from_non_constructors + changes;
                state_var_changed_from_non_constructors = new_changes;
            }
        }

        let mut state_var_changed_from_constructors =
            ApproximateStorageChangeFinder::default(context);

        for func in helpers::get_implemented_external_and_public_functions(context) {
            if *func.kind() != FunctionKind::Constructor {
                continue;
            }
            if func.compiles_for_solc_below_0_6_5(context) {
                // The immutable keyword was introduced in 0.6.5
                continue;
            }
            // In the case of constructors, we shouldn't explore the callgraph due to the reasons
            // stated in this detector's solidity test file
            state_var_changed_from_constructors =
                ApproximateStorageChangeFinder::from(context, func);
        }

        let collection_b = state_var_changed_from_non_constructors
            .fetch_non_exhaustive_manipulated_state_variables();
        let collection_b_ids: HashSet<_> = collection_b.into_iter().map(|v| v.id).collect();

        let collection_c =
            state_var_changed_from_constructors.fetch_non_exhaustive_manipulated_state_variables();
        let collection_c_ids: HashSet<_> = collection_c.into_iter().map(|v| v.id).collect();

        let collection_r1 = collection_c_ids
            .into_iter()
            .filter(|n| !collection_b_ids.contains(n))
            .collect::<HashSet<_>>();

        // Now to calculate collection_r2, we loop through collection_a and capture it if the ID is
        // found in collection_r1

        for state_variable in collection_a {
            if collection_r1.contains(&state_variable.id) {
                capture!(self, context, state_variable);
            }
        }

        Ok(!self.found_instances.is_empty())
    }

    fn severity(&self) -> IssueSeverity {
        IssueSeverity::Low
    }

    fn title(&self) -> String {
        String::from("State variable could be declared immutable")
    }

    fn description(&self) -> String {
        String::from("State variables that are should be declared immutable to save gas. Add the `immutable` attribute to state variables that are only changed in the constructor")
    }

    fn instances(&self) -> BTreeMap<(String, usize, String), NodeID> {
        self.found_instances.clone()
    }

    fn name(&self) -> String {
        format!(
            "{}",
            IssueDetectorNamePool::StateVariableCouldBeDeclaredImmutable
        )
    }
}

#[cfg(test)]
mod state_variable_could_be_immutable_tests {
    use serial_test::serial;

    use crate::detect::{
        detector::IssueDetector,
        low::state_variable_could_be_immutable::StateVariableCouldBeImmutableDetector,
    };

    #[test]
    #[serial]
    fn test_state_variable_could_be_declared_immutable() {
        let context = crate::detect::test_utils::load_solidity_source_unit(
            "../tests/adhoc-sol-files/CouldBeImmutable.sol",
        );

        let mut detector = StateVariableCouldBeImmutableDetector::default();
        let found = detector.detect(&context).unwrap();
        // assert that the detector found an issue
        assert!(found);
        // assert that the detector found the correct number of instances
        assert_eq!(detector.instances().len(), 1);
        println!("{:?}", detector.instances());
        // assert the severity is low
        assert_eq!(
            detector.severity(),
            crate::detect::detector::IssueSeverity::Low
        );
    }
}
