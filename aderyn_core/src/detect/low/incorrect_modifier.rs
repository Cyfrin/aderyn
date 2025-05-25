use std::{
    collections::{BTreeMap, HashMap},
    error::Error,
};

use crate::{
    ast::NodeID,
    capture,
    context::{
        browser::{ExtractFunctionCalls, ExtractPlaceholderStatements, ExtractRevertStatements},
        flow::{Cfg, CfgNodeId},
        workspace::WorkspaceContext,
    },
    detect::detector::{IssueDetector, IssueDetectorNamePool, IssueSeverity},
};
use eyre::Result;

#[derive(Default)]
pub struct IncorrectUseOfModifierDetector {
    // Keys are: [0] source file name, [1] line number, [2] character location of node.
    // Do not add items manually, use `capture!` to add nodes to this BTreeMap.
    found_instances: BTreeMap<(String, usize, String), NodeID>,
    hints: BTreeMap<(String, usize, String), String>,
}

impl IssueDetector for IncorrectUseOfModifierDetector {
    fn detect(&mut self, context: &WorkspaceContext) -> Result<bool, Box<dyn Error>> {
        // When you have found an instance of the issue,
        // use the following macro to add it to `found_instances`:
        //
        // capture!(self, context, item);
        // capture!(self, context, item, "hint");

        for modifier in context.modifier_definitions() {
            let Some((cfg, start, _)) = Cfg::from_modifier_body(context, modifier) else {
                continue;
            };

            if !all_paths_have_revert_or_placeholder(context, &cfg, start) {
                capture!(self, context, modifier);
            }
        }

        Ok(!self.found_instances.is_empty())
    }

    fn severity(&self) -> IssueSeverity {
        IssueSeverity::Low
    }

    fn title(&self) -> String {
        String::from("Incorrect Use Of Modifier")
    }

    fn description(&self) -> String {
        String::from("If a modifier does not execute `_` or revert, the execution of the function will return the default value, which can be misleading for the caller. It is recommended that all the paths in a modifier must execute _ or revert.")
    }

    fn instances(&self) -> BTreeMap<(String, usize, String), NodeID> {
        self.found_instances.clone()
    }

    fn hints(&self) -> BTreeMap<(String, usize, String), String> {
        self.hints.clone()
    }

    fn name(&self) -> String {
        IssueDetectorNamePool::IncorrectUseOfModifier.to_string()
    }
}

fn all_paths_have_revert_or_placeholder(
    context: &WorkspaceContext,
    cfg: &Cfg,
    start: CfgNodeId,
) -> bool {
    // First bit on signifies revert is present in the path so far
    // Second bit on signifies placeholder is present in the path so far
    // ----------------------------
    // value | placeholder | revert
    // ----------------------------
    // 00    | N           | N
    // 01    | N           | Y
    // 10    | Y           | N
    // 11    | Y           | Y
    // ----------------------------
    type SoFar = usize;

    let mut visited = Default::default();
    let mut answers = Default::default();

    fn _all_paths_have_revert_or_placeholder(
        context: &WorkspaceContext,
        cfg: &Cfg,
        visited: &mut HashMap<CfgNodeId, Vec<bool>>,
        answers: &mut HashMap<CfgNodeId, Vec<bool>>,
        curr_node: CfgNodeId,
        so_far: SoFar,
    ) -> bool {
        if let Some(visited_node) = visited.get(&curr_node) {
            if visited_node[so_far] {
                return answers.get(&curr_node).expect("answers corrupted!")[so_far];
            }
        }

        let curr_cfg_node = cfg.nodes.get(&curr_node).expect("cfg is incomplete!");

        let mut inc: SoFar = so_far;

        if let Some(curr_ast_node) = curr_cfg_node.reflect(context) {
            // Check for placeholders in the current node
            let placeholders_in_curr_node =
                ExtractPlaceholderStatements::from(curr_ast_node).extracted;

            if !placeholders_in_curr_node.is_empty() {
                inc |= 1 << 1;
            }

            if (1 & inc) == 0 {
                // Check for possibility of revert

                let func_calls = ExtractFunctionCalls::from(curr_ast_node).extracted;
                if !func_calls.is_empty() {
                    inc |= 1;
                }

                let revert_stmts = ExtractRevertStatements::from(curr_ast_node).extracted;
                if !revert_stmts.is_empty() {
                    inc |= 1;
                }
            }
        }

        // At least one of placeholders/revert is present
        if inc != 0 {
            let visited_node = visited.entry(curr_node).or_insert_with(|| vec![false; 4]);
            visited_node[so_far] = true;

            let answer_node = answers.entry(curr_node).or_insert_with(|| vec![false; 4]);
            answer_node[so_far] = true;

            return true;
        }

        let children = curr_node.children(cfg);

        if children.is_empty() {
            let visited_node = visited.entry(curr_node).or_insert_with(|| vec![false; 4]);
            visited_node[so_far] = true;

            let answer_node = answers.entry(curr_node).or_insert_with(|| vec![false; 4]);
            answer_node[so_far] = false;

            return false;
        }

        let mut answer = true;

        for child in children {
            answer &=
                _all_paths_have_revert_or_placeholder(context, cfg, visited, answers, child, inc);
        }

        let visited_node = visited.entry(curr_node).or_insert_with(|| vec![false; 4]);
        visited_node[so_far] = true;

        let answer_node = answers.entry(curr_node).or_insert_with(|| vec![false; 4]);
        answer_node[so_far] = answer;

        answer
    }

    _all_paths_have_revert_or_placeholder(context, cfg, &mut visited, &mut answers, start, 0);

    let final_answer = answers.entry(start).or_default();

    // '0' because we start from having nothing
    final_answer[0]
}

#[cfg(test)]
mod test_incorrect_modifier {

    use crate::detect::{
        detector::IssueDetector, low::incorrect_modifier::IncorrectUseOfModifierDetector,
    };

    #[test]

    fn test_incorrect_modifier_detector() {
        let context = crate::detect::test_utils::load_solidity_source_unit(
            "../tests/contract-playground/src/IncorrectModifier.sol",
        );
        let mut detector = IncorrectUseOfModifierDetector::default();
        let found = detector.detect(&context).unwrap();
        assert!(found);

        assert_eq!(detector.instances().len(), 2);
    }
}
