use std::{
    collections::{BTreeMap, BTreeSet, HashSet},
    error::Error,
};

use crate::{
    ast::NodeID,
    capture,
    context::{
        browser::{ExtractFunctionCalls, Peek},
        flow::{Cfg, CfgNodeDescriptor, CfgNodeId},
        workspace_context::WorkspaceContext,
    },
    detect::{
        detector::{IssueDetector, IssueDetectorNamePool, IssueSeverity},
        helpers,
    },
};
use eyre::{eyre, Result};

#[derive(Default)]
pub struct EmitAfterExternalCallDetector {
    // Keys are: [0] source file name, [1] line number, [2] character location of node.
    // Do not add items manually, use `capture!` to add nodes to this BTreeMap.
    found_instances: BTreeMap<(String, usize, String), NodeID>,
    hints: BTreeMap<(String, usize, String), String>,
}

impl IssueDetector for EmitAfterExternalCallDetector {
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
                let emit_sites = find_emit_sites(&cfg, external_call_site);
                let mut hint = "Emission happens at: ".to_string();
                let mut emissions_found = false;

                for emit_site in emit_sites {
                    // There is no way to tell is the state change took place after the event if
                    // both are found at the same place
                    if emit_site != external_call_site {
                        emissions_found = true;
                        let emit_cfg_node = cfg.nodes.get(&emit_site).expect("cfg is corrupted");

                        if let Some(emit_ast_node) = emit_cfg_node.reflect(context) {
                            if let Some(state_change_code) = emit_ast_node.peek(context) {
                                hint.push('`');
                                hint.push_str(&state_change_code);
                                hint.push('`');
                                hint.push_str(", ");
                            }
                        }
                    }
                }

                if emissions_found {
                    hint.pop();
                    hint.pop();

                    capture!(self, context, external_call_ast_node, hint);
                }
            }
        }

        Ok(!self.found_instances.is_empty())
    }

    fn severity(&self) -> IssueSeverity {
        IssueSeverity::Low
    }

    fn title(&self) -> String {
        String::from("External call is followed by an emit statement")
    }

    fn description(&self) -> String {
        String::from("In most cases it is a best practice to perform the emit before making an external call to avoid a potential read only re-entrancy attack.")
    }

    fn instances(&self) -> BTreeMap<(String, usize, String), NodeID> {
        self.found_instances.clone()
    }

    fn hints(&self) -> BTreeMap<(String, usize, String), String> {
        self.hints.clone()
    }

    fn name(&self) -> String {
        IssueDetectorNamePool::EmitAfterExternalCall.to_string()
    }
}

fn find_emit_sites(cfg: &Cfg, start_node: CfgNodeId) -> BTreeSet<CfgNodeId> {
    let mut visited = Default::default();
    let mut emit_statements_sites = Default::default();

    fn _find_emit_sites(
        cfg: &Cfg,
        visited: &mut HashSet<CfgNodeId>,
        curr_node: CfgNodeId,
        emit_sites: &mut HashSet<CfgNodeId>,
    ) -> Option<()> {
        if visited.contains(&curr_node) {
            return Some(());
        }

        visited.insert(curr_node);

        // Check if `curr_node` is an external call site
        let curr_cfg_node = cfg.nodes.get(&curr_node)?;

        if let CfgNodeDescriptor::EmitStatement(_) = &curr_cfg_node.nd {
            emit_sites.insert(curr_node);
        }

        // Continue the recursion
        for child in curr_node.children(cfg) {
            _find_emit_sites(cfg, visited, child, emit_sites);
        }

        Some(())
    }

    _find_emit_sites(cfg, &mut visited, start_node, &mut emit_statements_sites);

    emit_statements_sites.into_iter().collect()
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

            if function_calls.iter().any(|f| f.is_external_call()) {
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
mod emit_after_external_call_tests {
    use serial_test::serial;

    use crate::detect::{
        detector::IssueDetector, low::emit_after_ext_call::EmitAfterExternalCallDetector,
    };

    #[test]
    #[serial]
    fn test_emit_after_external_call() {
        let context = crate::detect::test_utils::load_solidity_source_unit(
            "../tests/contract-playground/src/EmitAfterExternalCall.sol",
        );

        let mut detector = EmitAfterExternalCallDetector::default();
        let found = detector.detect(&context).unwrap();
        // assert that the detector found an issue
        assert!(found);
        // assert that the detector found the correct number of instances
        assert_eq!(detector.instances().len(), 4);
        // assert the severity is high
        assert_eq!(detector.severity(), crate::detect::detector::IssueSeverity::Low);
    }
}
