use std::{
    collections::{BTreeMap, BTreeSet, HashSet},
    error::Error,
};

use crate::{ast::NodeID, context::browser::Peek};

use crate::{
    capture,
    context::{
        browser::{ApproximateStorageChangeFinder, ExtractFunctionCalls},
        flow::{Cfg, CfgNodeId},
        workspace::WorkspaceContext,
    },
    detect::{
        detector::{IssueDetector, IssueDetectorNamePool, IssueSeverity},
        helpers,
    },
};
use eyre::{eyre, Result};

#[derive(Default)]
pub struct ReentrancyStateChangeDetector {
    // Keys are: [0] source file name, [1] line number, [2] character location of node.
    // Do not add items manually, use `capture!` to add nodes to this BTreeMap.
    found_instances: BTreeMap<(String, usize, String), NodeID>,
    hints: BTreeMap<(String, usize, String), String>,
}

impl IssueDetector for ReentrancyStateChangeDetector {
    fn detect(&mut self, context: &WorkspaceContext) -> Result<bool, Box<dyn Error>> {
        // When you have found an instance of the issue,
        // use the following macro to add it to `found_instances`:
        //
        // capture!(self, context, item);
        // capture!(self, context, item, "hint");

        for func in helpers::get_implemented_external_and_public_functions(context) {
            let (cfg, start, _) =
                Cfg::from_function_body(context, func).ok_or(eyre!("corrupted function"))?;

            // Discover external calls
            let external_call_sites = find_external_call_sites(context, &cfg, start);

            // For each external call, figure out if it's followed by a state change
            for external_call_site in external_call_sites {
                // Capture the external call
                let external_call_cfg_node =
                    cfg.nodes.get(&external_call_site).expect("cfg is corrupted!");

                let Some(external_call_ast_node) = external_call_cfg_node.reflect(context) else {
                    continue;
                };

                // Discover state changes that follow the external call
                let state_changes = find_state_change_sites(context, &cfg, external_call_site);
                let mut hint = "State is changed at: ".to_string();
                let mut state_changes_found = false;

                for state_change in state_changes {
                    // There is no way to tell is the state change took place after the event if
                    // both are found at the same place
                    if state_change != external_call_site {
                        state_changes_found = true;
                        let state_change_cfg_node =
                            cfg.nodes.get(&state_change).expect("cfg is corrupted");

                        if let Some(state_change_ast_node) = state_change_cfg_node.reflect(context)
                        {
                            if let Some(state_change_code) = state_change_ast_node.peek(context) {
                                hint.push('`');
                                hint.push_str(&state_change_code);
                                hint.push('`');
                                hint.push_str(", ");
                            }
                        }
                    }
                }

                if state_changes_found {
                    hint.pop();
                    hint.pop();

                    capture!(self, context, external_call_ast_node, hint);
                }
            }
        }

        Ok(!self.found_instances.is_empty())
    }

    fn severity(&self) -> IssueSeverity {
        IssueSeverity::High
    }

    fn title(&self) -> String {
        String::from("Reentrancy: State change after external call")
    }

    fn description(&self) -> String {
        String::from(
            "Changing state after an external call can lead to re-entrancy attacks.\
        Use the checks-effects-interactions pattern to avoid this issue.",
        )
    }

    fn instances(&self) -> BTreeMap<(String, usize, String), NodeID> {
        self.found_instances.clone()
    }

    fn hints(&self) -> BTreeMap<(String, usize, String), String> {
        self.hints.clone()
    }

    fn name(&self) -> String {
        IssueDetectorNamePool::ReentrancyStateChange.to_string()
    }
}

fn find_state_change_sites(
    context: &WorkspaceContext,
    cfg: &Cfg,
    start_node: CfgNodeId,
) -> BTreeSet<CfgNodeId> {
    let mut visited = Default::default();
    let mut state_change_sites = Default::default();

    fn _find_following_state_change_sites(
        context: &WorkspaceContext,
        cfg: &Cfg,
        visited: &mut HashSet<CfgNodeId>,
        curr_node: CfgNodeId,
        state_change_sites: &mut HashSet<CfgNodeId>,
    ) -> Option<()> {
        if visited.contains(&curr_node) {
            return Some(());
        }

        visited.insert(curr_node);

        // Check if `curr_node` is an external call site
        let curr_cfg_node = cfg.nodes.get(&curr_node)?;

        // Grab the AST version of the Cfg Node
        if let Some(curr_ast_node) = curr_cfg_node.reflect(context) {
            let state_changes = ApproximateStorageChangeFinder::from(context, curr_ast_node);

            if state_changes.state_variables_have_been_manipulated() {
                state_change_sites.insert(curr_node);
            }
        }

        // Continue the recursion
        for child in curr_node.children(cfg) {
            _find_following_state_change_sites(context, cfg, visited, child, state_change_sites);
        }

        Some(())
    }

    _find_following_state_change_sites(
        context,
        cfg,
        &mut visited,
        start_node,
        &mut state_change_sites,
    );

    state_change_sites.into_iter().collect()
}

fn find_external_call_sites(
    context: &WorkspaceContext,
    cfg: &Cfg,
    start_node: CfgNodeId,
) -> BTreeSet<CfgNodeId> {
    let mut visited = Default::default();
    let mut external_call_sites = Default::default();

    fn _find_external_call_sites(
        context: &WorkspaceContext,
        cfg: &Cfg,
        visited: &mut HashSet<CfgNodeId>,
        curr_node: CfgNodeId,
        external_call_sites: &mut HashSet<CfgNodeId>,
    ) -> Option<()> {
        if visited.contains(&curr_node) {
            return Some(());
        }

        visited.insert(curr_node);

        // Check if `curr_node` is an external call site
        let curr_cfg_node = cfg.nodes.get(&curr_node)?;

        // Grab the AST version of the Cfg Node
        if let Some(curr_ast_node) = curr_cfg_node.reflect(context) {
            let function_calls = ExtractFunctionCalls::from(curr_ast_node).extracted;

            if function_calls.iter().any(|f| f.is_extcallish()) {
                external_call_sites.insert(curr_node);
            }
        }

        // Continue the recursion
        for child in curr_node.children(cfg) {
            _find_external_call_sites(context, cfg, visited, child, external_call_sites);
        }

        Some(())
    }

    _find_external_call_sites(context, cfg, &mut visited, start_node, &mut external_call_sites);

    external_call_sites.into_iter().collect()
}

#[cfg(test)]
mod state_change_after_external_call_tests {

    use crate::detect::{
        detector::IssueDetector, high::reentrancy_state_change::ReentrancyStateChangeDetector,
    };

    #[test]

    fn test_state_change_after_external_call() {
        let context = crate::detect::test_utils::load_solidity_source_unit(
            "../tests/contract-playground/src/StateChangeAfterExternalCall.sol",
        );

        let mut detector = ReentrancyStateChangeDetector::default();
        let found = detector.detect(&context).unwrap();
        assert!(found);
        assert_eq!(detector.instances().len(), 4);
    }
}
