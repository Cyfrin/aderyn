use std::{collections::BTreeMap, error::Error};

use crate::ast::NodeID;

use crate::{
    capture,
    context::{
        browser::{ExtractIdentifiers, ExtractRevertStatements},
        graph::{CallGraphConsumer, CallGraphDirection, CallGraphVisitor},
        workspace::WorkspaceContext,
    },
    detect::{
        detector::{IssueDetector, IssueDetectorNamePool, IssueSeverity},
        helpers::get_explore_centers_of_loops,
    },
};
use eyre::Result;

#[derive(Default)]
pub struct RequireRevertInLoopDetector {
    // Keys are: [0] source file name, [1] line number, [2] character location of node.
    // Do not add items manually, use `capture!` to add nodes to this BTreeMap.
    found_instances: BTreeMap<(String, usize, String), NodeID>,
}

impl IssueDetector for RequireRevertInLoopDetector {
    fn detect(&mut self, context: &WorkspaceContext) -> Result<bool, Box<dyn Error>> {
        let loop_explore_centers = get_explore_centers_of_loops(context);

        for l in loop_explore_centers {
            let callgraphs = CallGraphConsumer::get(context, &[l], CallGraphDirection::Inward)?;

            for callgraph in callgraphs {
                let mut tracker = RevertAndRequireTracker::default();
                callgraph.accept(context, &mut tracker)?;

                if tracker.has_require_or_revert || tracker.has_revert_statement {
                    capture!(self, context, l);
                }
            }
        }

        Ok(!self.found_instances.is_empty())
    }

    fn severity(&self) -> IssueSeverity {
        IssueSeverity::Low
    }

    fn title(&self) -> String {
        String::from("Loop Contains `require`/`revert`")
    }

    fn description(&self) -> String {
        String::from("Avoid `require` / `revert` statements in a loop because a single bad item can cause the whole transaction to fail. It's better to forgive on fail and return failed elements post processing of the loop")
    }

    fn instances(&self) -> BTreeMap<(String, usize, String), NodeID> {
        self.found_instances.clone()
    }

    fn name(&self) -> String {
        format!("{}", IssueDetectorNamePool::RequireRevertInLoop)
    }
}

#[derive(Default)]
struct RevertAndRequireTracker {
    has_require_or_revert: bool,
    has_revert_statement: bool,
}

impl CallGraphVisitor for RevertAndRequireTracker {
    fn visit_any(&mut self, node: &crate::ast::ASTNode) -> eyre::Result<()> {
        // Check for revert() and require() calls
        let identifiers = ExtractIdentifiers::from(node).extracted;

        let requires_and_reverts_are_present =
            identifiers.iter().any(|id| id.name == "revert" || id.name == "require");

        if requires_and_reverts_are_present {
            self.has_require_or_revert = true;
        }

        // Check for revert statements
        let revert_statements = ExtractRevertStatements::from(node).extracted;

        if !revert_statements.is_empty() {
            self.has_revert_statement = true;
        }
        Ok(())
    }
}

#[cfg(test)]
mod reevrts_and_requires_in_loops {

    use crate::detect::{
        detector::IssueDetector, low::require_revert_in_loop::RequireRevertInLoopDetector,
    };

    #[test]

    fn test_reverts_and_requires_in_loops_by_loading_contract_directly() {
        let context = crate::detect::test_utils::load_solidity_source_unit(
            "../tests/contract-playground/src/RevertsAndRequriesInLoops.sol",
        );

        let mut detector = RequireRevertInLoopDetector::default();
        let found = detector.detect(&context).unwrap();

        assert!(found);
        assert_eq!(detector.instances().len(), 2);
    }
}
