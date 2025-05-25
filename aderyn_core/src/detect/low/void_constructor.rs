use std::{collections::BTreeMap, error::Error};

use crate::ast::{ASTNode, FunctionKind, ModifierInvocationKind, NodeID};

use crate::{
    capture,
    context::workspace::WorkspaceContext,
    detect::detector::{IssueDetector, IssueDetectorNamePool, IssueSeverity},
};
use eyre::Result;

#[derive(Default)]
pub struct VoidConstructorDetector {
    // Keys are: [0] source file name, [1] line number, [2] character location of node.
    // Do not add items manually, use `capture!` to add nodes to this BTreeMap.
    found_instances: BTreeMap<(String, usize, String), NodeID>,
}

impl IssueDetector for VoidConstructorDetector {
    fn detect(&mut self, context: &WorkspaceContext) -> Result<bool, Box<dyn Error>> {
        // PLAN
        // Gather all the invocations of base constructors
        // For each, inspect the contract and see if there is a constructor defined. If there isn't,
        // capture the invocation

        for modifier_invocation in context.modifier_invocations() {
            if modifier_invocation.kind != Some(ModifierInvocationKind::BaseConstructorSpecifier) {
                continue;
            }
            if let Some(reference_declaration) = match &modifier_invocation.modifier_name {
                crate::ast::IdentifierOrIdentifierPath::Identifier(identifier) => {
                    identifier.referenced_declaration
                }
                crate::ast::IdentifierOrIdentifierPath::IdentifierPath(identifier_path) => {
                    Some(identifier_path.referenced_declaration)
                }
            } {
                if let Some(ASTNode::ContractDefinition(contract)) =
                    context.nodes.get(&reference_declaration)
                {
                    if contract
                        .function_definitions()
                        .into_iter()
                        .filter(|f| *f.kind() == FunctionKind::Constructor)
                        .count()
                        == 0
                    {
                        capture!(self, context, modifier_invocation);
                    }
                }
            }
        }

        Ok(!self.found_instances.is_empty())
    }

    fn severity(&self) -> IssueSeverity {
        IssueSeverity::Low
    }

    fn title(&self) -> String {
        String::from("Void constructor")
    }

    fn description(&self) -> String {
        String::from("Call to a constructor that is not implemented.")
    }

    fn instances(&self) -> BTreeMap<(String, usize, String), NodeID> {
        self.found_instances.clone()
    }

    fn name(&self) -> String {
        format!("{}", IssueDetectorNamePool::VoidConstructor)
    }
}

#[cfg(test)]
mod template_void_constructors {

    use crate::detect::{detector::IssueDetector, low::void_constructor::VoidConstructorDetector};

    #[test]

    fn test_template_detector() {
        let context = crate::detect::test_utils::load_solidity_source_unit(
            "../tests/contract-playground/src/VoidConstructor.sol",
        );

        let mut detector = VoidConstructorDetector::default();
        let found = detector.detect(&context).unwrap();

        assert!(found);
        assert_eq!(detector.instances().len(), 1);
    }
}
