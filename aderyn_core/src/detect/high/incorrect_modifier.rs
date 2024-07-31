use std::collections::BTreeMap;
use std::convert::identity;
use std::error::Error;

use crate::ast::{ASTNode, ContractKind, NodeID, NodeType};

use crate::capture;
use crate::context::browser::{
    ExtractIdentifiers, ExtractPlaceholderStatements, ExtractRevertStatements,
    GetClosestAncestorOfTypeX,
};
use crate::context::investigator::{
    StandardInvestigationStyle, StandardInvestigator, StandardInvestigatorVisitor,
};
use crate::detect::detector::IssueDetectorNamePool;
use crate::{
    context::workspace_context::WorkspaceContext,
    detect::detector::{IssueDetector, IssueSeverity},
};
use eyre::Result;

#[derive(Default)]
pub struct IncorrectModifierDetector {
    // Keys are: [0] source file name, [1] line number, [2] character location of node.
    // Do not add items manually, use `capture!` to add nodes to this BTreeMap.
    found_instances: BTreeMap<(String, usize, String), NodeID>,
}

impl IssueDetector for IncorrectModifierDetector {
    fn detect(&mut self, context: &WorkspaceContext) -> Result<bool, Box<dyn Error>> {
        for modifier in context.modifier_definitions() {
            if let Some(ASTNode::ContractDefinition(parent_contract)) =
                modifier.closest_ancestor_of_type(context, NodeType::ContractDefinition)
            {
                if parent_contract.kind != ContractKind::Contract
                    || parent_contract.is_abstract.is_some_and(identity)
                {
                    // Skip checking the modifier if it's part of an abstract contract
                    continue;
                }
            }

            let mut incorrect_modifier_tracker = PlaceholdersRequiresAndRevertsTracker::default();

            let investigator = StandardInvestigator::new(
                context,
                &[&(modifier.into())],
                StandardInvestigationStyle::Downstream,
            )?;

            investigator.investigate(context, &mut incorrect_modifier_tracker)?;

            if !incorrect_modifier_tracker.satisfied() {
                capture!(self, context, modifier);
            }
        }

        Ok(!self.found_instances.is_empty())
    }

    fn severity(&self) -> IssueSeverity {
        IssueSeverity::High
    }

    fn title(&self) -> String {
        String::from("Incorrect Modifier")
    }

    fn description(&self) -> String {
        String::from("A modifier must contain a placeholder statement `_` and a `revert` or a `require` condition.")
    }

    fn instances(&self) -> BTreeMap<(String, usize, String), NodeID> {
        self.found_instances.clone()
    }

    fn name(&self) -> String {
        IssueDetectorNamePool::IncorrectModifier.to_string()
    }
}

#[derive(Default, Debug)]
struct PlaceholdersRequiresAndRevertsTracker {
    has_placeholder: bool,
    has_require: bool,
    has_revert: bool,
}

impl StandardInvestigatorVisitor for PlaceholdersRequiresAndRevertsTracker {
    fn visit_any(&mut self, ast_node: &crate::ast::ASTNode) -> eyre::Result<()> {
        // If the case is already satisfied, don't bother checking
        if self.satisfied() {
            return Ok(());
        }

        // Handle placeholders ...
        if !self.has_placeholder {
            let place_holders = ExtractPlaceholderStatements::from(ast_node).extracted;
            self.has_placeholder = !place_holders.is_empty();
        }

        let identifiers = ExtractIdentifiers::from(ast_node).extracted;

        // Handle require
        if !self.has_require {
            let requires = identifiers.iter().filter(|id| id.name == "require").count();
            self.has_require = requires > 0;
        }

        // Handle revert (2 steps)
        // First, we check for `revert()` and then, if not found, we check for `revert MyError()`
        // In the former, revert is an identifier in a function call
        // In the latter, revert is a RevertStatement
        if !self.has_revert {
            let reverts = identifiers.iter().filter(|id| id.name == "revert").count();
            self.has_revert = reverts > 0;
        }

        if !self.has_revert {
            let revert_statements = ExtractRevertStatements::from(ast_node).extracted;
            self.has_revert = !revert_statements.is_empty();
        }

        Ok(())
    }
}

impl PlaceholdersRequiresAndRevertsTracker {
    pub fn satisfied(&self) -> bool {
        self.has_placeholder && (self.has_require || self.has_revert)
    }
}

#[cfg(test)]
mod incorrect_modifier_detector_tests {
    use serial_test::serial;

    use crate::detect::{
        detector::IssueDetector, high::incorrect_modifier::IncorrectModifierDetector,
    };

    #[test]
    #[serial]
    fn test_incorrect_modifier() {
        let context = crate::detect::test_utils::load_solidity_source_unit_with_callgraphs(
            "../tests/contract-playground/src/IncorrectModifier.sol",
        );

        let mut detector = IncorrectModifierDetector::default();
        let found = detector.detect(&context).unwrap();

        println!("{:#?}", detector.instances());

        // assert that the detector found an issue
        assert!(found);
        // assert that the detector found the correct number of instances
        assert_eq!(detector.instances().len(), 3);
        // assert the severity is high
        assert_eq!(
            detector.severity(),
            crate::detect::detector::IssueSeverity::High
        );
    }
}
