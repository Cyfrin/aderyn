use std::{
    collections::{BTreeMap, HashSet},
    error::Error,
};

use crate::{
    ast::{NodeID, NodeType},
    capture,
    context::{
        flow::{Cfg, CfgNodeId},
        workspace::WorkspaceContext,
    },
    detect::detector::{IssueDetector, IssueDetectorNamePool, IssueSeverity},
};
use eyre::Result;

// HOW TO USE THIS TEMPLATE:
// 1. Copy this file and rename it to the snake_case version of the issue you are detecting.
// 2. Rename the TemplateDetector struct and impl to your new issue name.
// 3. Add this file and detector struct to the mod.rs file in the same directory.
// 4. Implement the detect function to find instances of the issue.

#[derive(Default)]
pub struct MultiplePlaceholdersDetector {
    // Keys are: [0] source file name, [1] line number, [2] character location of node.
    // Do not add items manually, use `capture!` to add nodes to this BTreeMap.
    found_instances: BTreeMap<(String, usize, String), NodeID>,
    hints: BTreeMap<(String, usize, String), String>,
}

impl IssueDetector for MultiplePlaceholdersDetector {
    fn detect(&mut self, context: &WorkspaceContext) -> Result<bool, Box<dyn Error>> {
        let multiple_placeholders = |cfg: &Cfg, start: CfgNodeId| -> bool {
            // collect starting points
            let placeholders: HashSet<CfgNodeId> = {
                let mut set: HashSet<CfgNodeId> = Default::default();
                fn collect_cfg_nodes(
                    cfg: &Cfg,
                    visited: &mut HashSet<CfgNodeId>,
                    curr_node: CfgNodeId,
                ) {
                    if visited.contains(&curr_node) {
                        return;
                    }
                    visited.insert(curr_node);

                    for child in curr_node.children(cfg) {
                        collect_cfg_nodes(cfg, visited, child);
                    }
                }
                collect_cfg_nodes(cfg, &mut set, start);
                set.into_iter()
                    .filter(|n| {
                        cfg.nodes.get(n).is_some_and(|c| {
                            c.reflect(context)
                                .is_some_and(|d| d.node_type() == NodeType::PlaceholderStatement)
                        })
                    })
                    .collect()
            };

            fn dfs(
                context: &WorkspaceContext,
                cfg: &Cfg,
                visited: &mut HashSet<CfgNodeId>,
                curr_node: CfgNodeId,
                count: &mut usize,
            ) {
                if visited.contains(&curr_node) {
                    return;
                }
                visited.insert(curr_node);

                if cfg.nodes.get(&curr_node).is_some_and(|c| {
                    c.reflect(context)
                        .is_some_and(|d| d.node_type() == NodeType::PlaceholderStatement)
                }) {
                    *count += 1
                }

                for child in curr_node.children(cfg) {
                    dfs(context, cfg, visited, child, count);
                }
            }

            for starting_point in placeholders {
                let mut visited: HashSet<CfgNodeId> = Default::default();
                let mut count = 0;
                dfs(context, cfg, &mut visited, starting_point, &mut count);
                if count > 1 {
                    return true;
                }
            }
            false
        };

        for modifier in context.modifier_definitions() {
            let Some((cfg, start, _)) = Cfg::from_modifier_body(context, modifier) else {
                continue;
            };
            if multiple_placeholders(&cfg, start) {
                capture!(self, context, modifier);
            }
        }
        Ok(!self.found_instances.is_empty())
    }

    fn severity(&self) -> IssueSeverity {
        IssueSeverity::Low
    }

    fn title(&self) -> String {
        String::from("Multiple Placeholders in Modifier")
    }

    fn description(&self) -> String {
        String::from("Design the modifier to only contain 1 placeholder statement. If that is not possible, split the logic into multiple modifiers.")
    }

    fn instances(&self) -> BTreeMap<(String, usize, String), NodeID> {
        self.found_instances.clone()
    }

    fn hints(&self) -> BTreeMap<(String, usize, String), String> {
        self.hints.clone()
    }

    fn name(&self) -> String {
        IssueDetectorNamePool::MultiplePlaceholders.to_string()
    }
}

#[cfg(test)]
mod multiple_placeholder_tests {

    use crate::detect::{
        detector::IssueDetector, low::multiple_placeholders::MultiplePlaceholdersDetector,
        test_utils,
    };

    #[test]
    fn test_multiple_placeholders() {
        let context = test_utils::load_solidity_source_unit(
            "../tests/contract-playground/src/MultiplePlaceholders.sol",
        );

        let mut detector = MultiplePlaceholdersDetector::default();
        let found = detector.detect(&context).unwrap();
        assert!(found);
        assert_eq!(detector.instances().len(), 3);
    }
}
