use std::collections::{BTreeMap, HashSet};
use std::error::Error;

use crate::ast::NodeID;

use crate::capture;
use crate::context::browser::ExtractMemberAccesses;
use crate::context::investigator::{
    StandardInvestigationStyle, StandardInvestigator, StandardInvestigatorVisitor,
};

use crate::context::workspace_context::ASTNode;
use crate::detect::detector::IssueDetectorNamePool;
use crate::{
    context::workspace_context::WorkspaceContext,
    detect::detector::{IssueDetector, IssueSeverity},
};
use eyre::Result;

#[derive(Default)]
pub struct SubWithoutIfDetector {
    // Keys are: [0] source file name, [1] line number, [2] character location of node.
    // Do not add items manually, use `capture!` to add nodes to this BTreeMap.
    found_instances: BTreeMap<(String, usize, String), NodeID>,
}

impl IssueDetector for SubWithoutIfDetector {
    fn detect(&mut self, context: &WorkspaceContext) -> Result<bool, Box<dyn Error>> {
        // Plan:
        // First, collect the if statements and investigate them for any `X.sub(Y)` patterns
        // and capture all the IDs ... Set A (protected_subs)
        // Then, collect all the IDS of X.sub(Y) pattern ... Set B (all_subs)
        // Answer = SetB - SetA
        // (We're left with unchecked sub method calls)

        let mut all_subs = context
            .member_accesses()
            .into_iter()
            .filter(|member| member.member_name == "sub")
            .map(|x| x.id)
            .collect::<HashSet<_>>();

        let mut protected_subs: HashSet<NodeID> = HashSet::new();

        let if_statements = context
            .if_statements()
            .into_iter()
            .map(|if_statement| if_statement.into())
            .collect::<Vec<ASTNode>>();

        let mut tracker = SubTracker {
            sub_pattern_ids: vec![],
        };
        let investigator = StandardInvestigator::new(
            context,
            &if_statements.iter().collect::<Vec<_>>(),
            StandardInvestigationStyle::Downstream,
        )?;
        investigator.investigate(context, &mut tracker)?;
        protected_subs.extend(tracker.sub_pattern_ids.iter());

        for protected_sub in protected_subs {
            let _ = all_subs.remove(&protected_sub);
        }

        // Now, we have the unprotected subs remaining in all_subs
        for sub in all_subs {
            context
                .nodes
                .get(&sub)
                .inspect(|&node| capture!(self, context, node));
        }

        Ok(!self.found_instances.is_empty())
    }

    fn severity(&self) -> IssueSeverity {
        IssueSeverity::Low
    }

    fn title(&self) -> String {
        String::from("Potential unchecked call made to `x.sub(y)`.")
    }

    fn description(&self) -> String {
        String::from("Use an if-statement to see that `x` is bigger than `y` to protect from unexpected reverts")
    }

    fn instances(&self) -> BTreeMap<(String, usize, String), NodeID> {
        self.found_instances.clone()
    }

    fn name(&self) -> String {
        IssueDetectorNamePool::SubWithoutIf.to_string()
    }
}

struct SubTracker {
    sub_pattern_ids: Vec<NodeID>,
}

impl StandardInvestigatorVisitor for SubTracker {
    fn visit_any(&mut self, node: &crate::context::workspace_context::ASTNode) -> eyre::Result<()> {
        let member_accesses = ExtractMemberAccesses::from(node).extracted;
        self.sub_pattern_ids.extend(
            member_accesses
                .into_iter()
                .filter(|member| member.member_name == "sub")
                .map(|x| x.id),
        );
        Ok(())
    }
}

#[cfg(test)]
mod sub_without_if_tests {
    use crate::detect::{detector::IssueDetector, low::sub_without_if::SubWithoutIfDetector};

    #[test]
    fn test_sub_without_if() {
        let context = crate::detect::test_utils::load_solidity_source_unit_with_callgraphs(
            "../tests/contract-playground/src/SubWithoutIf.sol",
        );

        let mut detector = SubWithoutIfDetector::default();
        let found = detector.detect(&context).unwrap();
        // assert that the detector found an issue
        assert!(found);
        // assert that the detector found the correct number of instances
        assert_eq!(detector.instances().len(), 1);
        // assert the severity is low
        assert_eq!(
            detector.severity(),
            crate::detect::detector::IssueSeverity::Low
        );
        // assert the title is correct
        assert_eq!(
            detector.title(),
            String::from("Potential unchecked call made to `x.sub(y)`.")
        );
        // assert the description is correct
        assert_eq!(
            detector.description(),
            String::from("Use an if-statement to see that `x` is bigger than `y` to protect from unexpected reverts"),
        );
    }
}
