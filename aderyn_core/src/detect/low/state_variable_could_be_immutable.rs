use std::{collections::BTreeMap, error::Error};

use crate::ast::{FunctionKind, Mutability, NodeID};

use crate::{
    capture,
    context::{browser::ApproximateStorageChangeFinder, workspace::WorkspaceContext},
    detect::{
        detector::{IssueDetector, IssueDetectorNamePool, IssueSeverity},
        helpers,
    },
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
            // If it's already marked immutable, ignore it!
            if variable.mutability() == Some(&Mutability::Immutable) {
                continue;
            }

            // Doesn't make sense to look for possible immutability if it's already declared
            // constant
            if variable.mutability() == Some(&Mutability::Constant) {
                continue;
            }

            // If the variable has already been initialized at it's definition then, later when
            // it's changed in the constructor, it cannot be marked immutable.
            //
            // This condition is opposite for detecting potentially constant variables. Over there,
            // we had to make sure that variable _had_ a value at the time of initializing.
            if variable.value.is_some() {
                continue;
            }

            // Do not report it if it's a struct / mapping
            if variable.type_descriptions.type_string.as_ref().is_some_and(|type_string| {
                type_string.starts_with("mapping") || type_string.starts_with("struct")
            }) {
                continue;
            }

            if variable.overrides.is_some() {
                continue;
            }

            if variable.state_variable && !variable.constant {
                collection_a.push(variable);
            }
        }

        let mut state_var_changed_from_non_constructors = None;
        let mut state_var_changed_from_constructors = None;

        // Gather the state changes that happen from non constructor functions
        for func in helpers::get_implemented_external_and_public_functions(context) {
            if *func.kind() == FunctionKind::Constructor {
                continue;
            }
            // Uses callgraph to explore inward
            if let Some(delta) = func.state_variable_changes(context) {
                if let Some(changes) = state_var_changed_from_non_constructors {
                    let new_changes = delta + changes;
                    state_var_changed_from_non_constructors = Some(new_changes);
                } else {
                    state_var_changed_from_non_constructors = Some(delta);
                }
            }
        }

        // Gather state changes that happen from constructor function only
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
            if let Some(changes) = state_var_changed_from_constructors {
                let new_changes = ApproximateStorageChangeFinder::from(context, func) + changes;
                state_var_changed_from_constructors = Some(new_changes);
            } else {
                state_var_changed_from_constructors =
                    Some(ApproximateStorageChangeFinder::from(context, func));
            }
        }

        // Collection A intersection with (collection C - collection B)
        if let (Some(collection_b), Some(collection_c)) =
            (state_var_changed_from_non_constructors, state_var_changed_from_constructors)
        {
            let collection_c = collection_c.fetch_non_exhaustive_manipulated_state_variables();
            let collection_b = collection_b.fetch_non_exhaustive_manipulated_state_variables();
            for state_variable in collection_a {
                if collection_c.contains(&state_variable) && !collection_b.contains(&state_variable)
                {
                    capture!(self, context, state_variable);
                }
            }
        }

        Ok(!self.found_instances.is_empty())
    }

    fn severity(&self) -> IssueSeverity {
        IssueSeverity::Low
    }

    fn title(&self) -> String {
        String::from("State Variable Could Be Immutable")
    }

    fn description(&self) -> String {
        String::from("State variables that are only changed in the constructor should be declared immutable to save gas. Add the `immutable` attribute to state variables that are only changed in the constructor")
    }

    fn instances(&self) -> BTreeMap<(String, usize, String), NodeID> {
        self.found_instances.clone()
    }

    fn name(&self) -> String {
        format!("{}", IssueDetectorNamePool::StateVariableCouldBeImmutable)
    }
}

#[cfg(test)]
mod state_variable_could_be_immutable_tests {

    use crate::detect::{
        detector::IssueDetector,
        low::state_variable_could_be_immutable::StateVariableCouldBeImmutableDetector,
    };

    #[test]

    fn test_state_variable_could_be_declared_immutable() {
        let context = crate::detect::test_utils::load_solidity_source_unit(
            "../tests/contract-playground/src/StateVariableCouldBeDeclaredImmutable.sol",
        );

        let mut detector = StateVariableCouldBeImmutableDetector::default();
        let found = detector.detect(&context).unwrap();
        assert!(found);
        assert_eq!(detector.instances().len(), 2);
    }
}
