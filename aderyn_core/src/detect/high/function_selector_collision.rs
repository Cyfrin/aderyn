use std::{
    collections::{BTreeMap, HashMap},
    error::Error,
};

use crate::ast::NodeID;

use crate::{
    capture,
    context::workspace::WorkspaceContext,
    detect::detector::{IssueDetector, IssueDetectorNamePool, IssueSeverity},
};
use eyre::Result;

#[derive(Default)]
pub struct FunctionSelectorCollisionDetector {
    // Keys are: [0] source file name, [1] line number, [2] character location of node.
    // Do not add items manually, use `capture!` to add nodes to this BTreeMap.
    found_instances: BTreeMap<(String, usize, String), NodeID>,
    hints: BTreeMap<(String, usize, String), String>,
}

impl IssueDetector for FunctionSelectorCollisionDetector {
    fn detect(&mut self, context: &WorkspaceContext) -> Result<bool, Box<dyn Error>> {
        // function_selector -> (function_name -> function_id)
        let mut selectors: HashMap<String, HashMap<String, Vec<NodeID>>> = HashMap::new();

        // PLAN
        // If we have > 1 function_name entries for any function_selector, then capture all the
        // corresponding NodeIDs

        for function in context.function_definitions() {
            if let Some(selector) = function.function_selector.as_ref() {
                let name = &function.name;
                match selectors.entry(selector.clone()) {
                    std::collections::hash_map::Entry::Occupied(mut o) => {
                        match o.get_mut().entry(name.clone()) {
                            std::collections::hash_map::Entry::Occupied(mut o) => {
                                o.get_mut().push(function.id);
                            }
                            std::collections::hash_map::Entry::Vacant(v) => {
                                v.insert(vec![function.id]);
                            }
                        };
                    }
                    std::collections::hash_map::Entry::Vacant(v) => {
                        let mut nested_entry = HashMap::new();
                        nested_entry.insert(name.clone(), vec![function.id]);
                        v.insert(nested_entry);
                    }
                }
            }
        }

        for function_entries in selectors.values() {
            if function_entries.len() >= 2 {
                // Now we know that there is a collision + at least 2 different function names found
                // for that selector.

                // Capture the relevant functions
                for (function_name, function_ids) in function_entries {
                    // Prepare data for the hint
                    let all_colliding_function_names = function_entries
                        .keys()
                        .filter(|&x| x != function_name)
                        .cloned()
                        .collect::<Vec<_>>();

                    for function_id in function_ids {
                        // Prepare the hint
                        let mut hint =
                            String::from("collides with the following function name(s) in scope: ");
                        hint.push_str(&all_colliding_function_names.join(", "));

                        // Capture the function
                        if let Some(node) = context.nodes.get(function_id) {
                            capture!(self, context, node, hint);
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
        String::from("Function Selector Collision")
    }

    fn description(&self) -> String {
        String::from("Function selector collides with other functions. This may cause the solidity function dispatcher to invoke the wrong function if the functions happen to be included in the same contract through an inheritance hierarchy later down the line. It is recommended to rename this function or change its parameters.")
    }

    fn instances(&self) -> BTreeMap<(String, usize, String), NodeID> {
        self.found_instances.clone()
    }

    fn hints(&self) -> BTreeMap<(String, usize, String), String> {
        self.hints.clone()
    }

    fn name(&self) -> String {
        format!("{}", IssueDetectorNamePool::FunctionSelectorCollision)
    }
}

#[cfg(test)]
mod function_signature_collision {

    use crate::detect::{
        detector::IssueDetector,
        high::function_selector_collision::FunctionSelectorCollisionDetector,
    };

    #[test]

    fn test_function_signature_collision() {
        let context = crate::detect::test_utils::load_solidity_source_unit(
            "../tests/contract-playground/src/FunctionSignatureCollision.sol",
        );

        let mut detector = FunctionSelectorCollisionDetector::default();
        let found = detector.detect(&context).unwrap();
        assert!(found);
        assert_eq!(detector.instances().len(), 2);
    }
}
