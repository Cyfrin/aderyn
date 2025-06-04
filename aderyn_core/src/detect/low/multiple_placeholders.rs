use std::{
    collections::{BTreeMap, HashMap},
    error::Error,
};

use crate::ast::NodeID;

use crate::{
    capture,
    context::{
        browser::ExtractPlaceholderStatements,
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
            let mut placeholders: HashMap<CfgNodeId, usize> = Default::default();

            fn dfs(
                context: &WorkspaceContext,
                cfg: &Cfg,
                placeholder_count: &mut HashMap<CfgNodeId, usize>,
                curr_node: CfgNodeId,
            ) -> usize {
                if let Some(count) = placeholder_count.get(&curr_node) {
                    return *count;
                }

                let self_pcount = {
                    let curr_cfg_node = cfg.nodes.get(&curr_node).expect("cfg is incomplete!");
                    if let Some(curr_ast_node) = curr_cfg_node.reflect(context) {
                        ExtractPlaceholderStatements::from(curr_ast_node).extracted.len()
                    } else {
                        0
                    }
                };

                let children_pcount = curr_node.children(cfg).iter().fold(0, |acc, curr| {
                    usize::max(acc, dfs(context, cfg, placeholder_count, *curr))
                });

                let total_pcount = self_pcount + children_pcount;

                placeholder_count.insert(curr_node, total_pcount);
                total_pcount
            }

            dfs(context, cfg, &mut placeholders, start);
            placeholders.get(&start).is_some_and(|count| *count > 1)
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
    };

    #[test]

    fn multiple_placeholders_test() {
        let context = crate::detect::test_utils::load_solidity_source_unit(
            "../tests/contract-playground/src/MultiplePlaceholders.sol",
        );

        let mut detector = MultiplePlaceholdersDetector::default();
        let found = detector.detect(&context).unwrap();
        assert!(found);
        assert_eq!(detector.instances().len(), 1);
    }
}
