use std::{collections::BTreeMap, error::Error};

use crate::{
    ast::{NodeID, NodeType},
    capture,
    context::{
        browser::{GetImmediateChildren, GetNextSibling, SortNodeReferencesToSequence},
        workspace_context::WorkspaceContext,
    },
    detect::detector::{IssueDetector, IssueDetectorNamePool, IssueSeverity},
};
use eyre::Result;

#[derive(Default)]
pub struct WrongOrderOfLayoutDetector {
    // Keys are source file name and line number
    found_instances: BTreeMap<(String, usize, String), NodeID>,
}

impl IssueDetector for WrongOrderOfLayoutDetector {
    fn detect(&mut self, context: &WorkspaceContext) -> Result<bool, Box<dyn Error>> {
        for contract_defs in context.contract_definitions() {
            if let Some(children) = contract_defs.children(context) {
                if let Some(sorted) = children.sort_by_src_position(context) {
                    for (index, &sorted_child_node) in sorted.iter().enumerate() {
                        if sorted_child_node.node_type() == NodeType::StructDefinition && index != 0
                        {
                            capture!(self, context, sorted_child_node);
                        }
                        if sorted_child_node.node_type() == NodeType::VariableDeclaration
                            && sorted_child_node.next_sibling(context).is_some_and(|n| {
                                n.node_type() != NodeType::VariableDeclaration
                                    && n.node_type() != NodeType::EventDefinition
                            })
                        {
                            capture!(self, context, sorted_child_node);
                        }
                        if sorted_child_node.node_type() == NodeType::EventDefinition
                            && sorted_child_node.next_sibling(context).is_some_and(|n| {
                                n.node_type() != NodeType::EventDefinition
                                    && n.node_type() != NodeType::ErrorDefinition
                            })
                        {
                            capture!(self, context, sorted_child_node);
                        }
                        if sorted_child_node.node_type() == NodeType::ErrorDefinition
                            && sorted_child_node.next_sibling(context).is_some_and(|n| {
                                n.node_type() != NodeType::ModifierDefinition
                                    && n.node_type() != NodeType::ErrorDefinition
                            })
                        {
                            capture!(self, context, sorted_child_node);
                        }
                        if sorted_child_node.node_type() == NodeType::ModifierDefinition
                            && sorted_child_node.next_sibling(context).is_some_and(|n| {
                                n.node_type() != NodeType::ModifierDefinition
                                    && n.node_type() != NodeType::FunctionDefinition
                            })
                        {
                            capture!(self, context, sorted_child_node);
                        }
                        if sorted_child_node.node_type() == NodeType::FunctionDefinition
                            && sorted_child_node
                                .next_sibling(context)
                                .is_some_and(|n| n.node_type() != NodeType::FunctionDefinition)
                        {
                            capture!(self, context, sorted_child_node);
                        }
                    }
                }
            }
        }

        Ok(!self.found_instances.is_empty())
    }

    fn title(&self) -> String {
        String::from("Contract elements should be laid out in the following order: `Type declarations` --> `State variables` --> `Events` --> `Errors` --> `Modifiers` --> `Functions`")
    }

    fn description(&self) -> String {
        String::from("Contract elements should be laid out according to the order specified in https://docs.soliditylang.org/en/latest/style-guide.html#order-of-layout")
    }

    fn severity(&self) -> IssueSeverity {
        IssueSeverity::Low
    }

    fn instances(&self) -> BTreeMap<(String, usize, String), NodeID> {
        self.found_instances.clone()
    }

    fn name(&self) -> String {
        format!("{}", IssueDetectorNamePool::WrongOrderOfLayout)
    }
}

#[cfg(test)]
mod wrong_order_of_layout_tests {
    use crate::detect::detector::{detector_test_helpers::load_contract, IssueDetector};

    use super::WrongOrderOfLayoutDetector;

    #[test]
    fn test_wrong_order_of_layout() {
        let context = load_contract(
            "../tests/contract-playground/out/WrongOrderOfLayout.sol/WrongOrderOfLayout.json",
        );

        let mut detector = WrongOrderOfLayoutDetector::default();
        // assert that the detector finds the public Function
        let found = detector.detect(&context).unwrap();
        assert!(found);
        // assert that the detector returns the correct number of instances
        assert_eq!(detector.instances().len(), 5);
        // assert that the detector returns the correct severity
        assert_eq!(
            detector.severity(),
            crate::detect::detector::IssueSeverity::Low
        );
        // assert that the detector returns the correct title
        assert_eq!(
            detector.title(),
            String::from("Contract elements should be laid out in the following order: `Type declarations` --> `State variables` --> `Events` --> `Errors` --> `Modifiers` --> `Functions`")
        );
        // assert that the detector returns the correct description
        assert_eq!(detector.description(), String::from("Contract elements should be laid out according to the order specified in https://docs.soliditylang.org/en/latest/style-guide.html#order-of-layout"));
    }
}
