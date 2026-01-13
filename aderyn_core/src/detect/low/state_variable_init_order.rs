use crate::{
    ast::{ExtractFunctionCalls, ExtractReferencedDeclarations, FunctionKind, NodeID},
    capture,
    context::{
        browser::ApproximateStorageChangeFinder,
        graph::{CallGraphConsumer, CallGraphDirection, CallGraphVisitor},
        workspace::WorkspaceContext,
    },
    detect::detector::{IssueDetector, IssueDetectorNamePool, IssueSeverity},
};
use eyre::Result;
use std::{
    collections::{BTreeMap, HashMap, HashSet, hash_map::Entry},
    error::Error,
};

#[derive(Default)]
pub struct StateVariableInitOrderDetector {
    found_instances: BTreeMap<(String, usize, String), NodeID>,
    hints: BTreeMap<(String, usize, String), String>,
}

impl IssueDetector for StateVariableInitOrderDetector {
    fn detect(&mut self, context: &WorkspaceContext) -> Result<bool, Box<dyn Error>> {
        if context.via_ir {
            for c in context.deployable_contracts() {
                // Gather State Variable IDs that are manipulated by constructors
                // in the contracts present in the inheritance.
                //
                // For each function call initializing state variable:
                //  - Find storage variables read / write in the corresponding function
                //  - If the storage variable is found in the above set, flag the state variable
                //
                // Map from
                //  State variable ID -> Vec<Contract IDs>
                //  where constructors of Contract IDs have written to State Variable ID
                let mut state_vars_written: HashMap<NodeID, HashSet<NodeID>> = HashMap::new();
                {
                    for contract in c.c3(context) {
                        for func in contract.function_definitions() {
                            if *func.kind() == FunctionKind::Constructor {
                                let state_change_finder =
                                    ApproximateStorageChangeFinder::from(context, func);

                                for v in state_change_finder
                                    .fetch_non_exhaustive_manipulated_state_variables()
                                {
                                    match state_vars_written.entry(v.id) {
                                        Entry::Occupied(mut o) => {
                                            o.get_mut().insert(contract.id);
                                        }
                                        Entry::Vacant(v) => {
                                            v.insert(HashSet::from([contract.id]));
                                        }
                                    }
                                }
                            }
                        }
                    }
                }

                for contract in c.c3(context) {
                    for var in contract.top_level_variables() {
                        let func_calls = ExtractFunctionCalls::from(var).extracted;
                        for func_call in &func_calls {
                            let callgraph = CallGraphConsumer::get_legacy(
                                context,
                                &[&func_call.into()],
                                CallGraphDirection::Inward,
                            )?;

                            let mut tracker = StorageVariableTracker::default();
                            callgraph.accept(context, &mut tracker)?;

                            for reference in tracker.all_references.iter() {
                                // Contract IDs whose constructor wrote to that state_variable
                                let Some(contract_ids) = state_vars_written.get(reference) else {
                                    continue;
                                };
                                if contract_ids.iter().filter(|&&id| id != contract.id).count() > 0
                                {
                                    capture!(self, context, var);
                                }
                            }
                        }
                    }
                }
            }
        }

        Ok(!self.found_instances.is_empty())
    }

    fn severity(&self) -> IssueSeverity {
        IssueSeverity::Low
    }

    fn title(&self) -> String {
        String::from("State variable's initial value relies on constructor of another contract.")
    }

    fn description(&self) -> String {
        String::from(
            "With via_ir flag is enabled, there is different behavior in contracts where initial value \
            of a state variable relies on the result of the constructor in another contract.\
            \nhttps://docs.soliditylang.org/en/latest/ir-breaking-changes.html#semantic-only-changes",
        )
    }

    fn instances(&self) -> BTreeMap<(String, usize, String), NodeID> {
        self.found_instances.clone()
    }

    fn hints(&self) -> BTreeMap<(String, usize, String), String> {
        self.hints.clone()
    }

    fn name(&self) -> String {
        IssueDetectorNamePool::StateVariableInitOrder.to_string()
    }
}

#[derive(Default)]
struct StorageVariableTracker {
    all_references: HashSet<NodeID>,
}

impl CallGraphVisitor for StorageVariableTracker {
    fn visit_any(&mut self, node: &crate::ast::ASTNode) -> eyre::Result<()> {
        let identifiers = ExtractReferencedDeclarations::from(node).extracted;
        self.all_references.extend(identifiers);
        Ok(())
    }
}

#[cfg(test)]
mod state_variable_init_order_tests {
    use super::*;

    #[test]
    fn test_state_variable_init_ordering() {
        let context = crate::detect::test_utils::load_solidity_source_unit(
            "../tests/via-ir-enabled/src/SemanticOrdering.sol",
        );

        let mut detector = StateVariableInitOrderDetector::default();
        let found = detector.detect(&context).unwrap();
        assert!(found);
        assert_eq!(detector.instances().len(), 1);
    }
}
