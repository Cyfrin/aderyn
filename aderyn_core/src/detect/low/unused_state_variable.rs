use std::collections::{BTreeMap, BTreeSet};
use std::convert::identity;
use std::error::Error;

use crate::ast::{ASTNode, ContractKind, NodeID, NodeType, Visibility};

use crate::capture;
use crate::context::browser::{
    ExtractReferencedDeclarations, ExtractVariableDeclarations, GetClosestAncestorOfTypeX,
};
use crate::detect::detector::IssueDetectorNamePool;
use crate::{
    context::workspace_context::WorkspaceContext,
    detect::detector::{IssueDetector, IssueSeverity},
};
use eyre::Result;

#[derive(Default)]
pub struct UnusedStateVariablesDetector {
    // Keys are: [0] source file name, [1] line number, [2] character location of node.
    // Do not add items manually, use `capture!` to add nodes to this BTreeMap.
    found_instances: BTreeMap<(String, usize, String), NodeID>,
}

impl IssueDetector for UnusedStateVariablesDetector {
    fn detect(&mut self, context: &WorkspaceContext) -> Result<bool, Box<dyn Error>> {
        // Collect all referencedDeclaration IDs adn StateVariableDeclarationIDs
        let mut all_referenced_declarations = BTreeSet::new();
        let mut all_state_variable_declarations = BTreeSet::new();

        for source_unit in context.source_units() {
            let referenced_declarations =
                ExtractReferencedDeclarations::from(source_unit).extracted;
            all_referenced_declarations.extend(referenced_declarations);
            let variable_declarations = ExtractVariableDeclarations::from(source_unit).extracted;
            all_state_variable_declarations.extend(
                variable_declarations
                    .into_iter()
                    .filter(|v| {
                        v.state_variable
                            && (v.visibility == Visibility::Private
                                || v.visibility == Visibility::Internal)
                    })
                    .map(|v| v.id),
            )
        }

        // Now, retain only the ones that have not been referenced
        all_state_variable_declarations.retain(|v| !all_referenced_declarations.contains(v));

        for unused_state_var_id in all_state_variable_declarations {
            if let Some(node) = context.nodes.get(&unused_state_var_id) {
                if let Some(ASTNode::ContractDefinition(contract)) =
                    node.closest_ancestor_of_type(context, NodeType::ContractDefinition)
                {
                    // If this variable is defined inside a contract, make sure it's not an abstract contract before capturing it
                    if !contract.is_abstract.is_some_and(identity)
                        && contract.kind == ContractKind::Contract
                    {
                        capture!(self, context, node);
                    }
                } else {
                    // Otherwise, just capture it !
                    capture!(self, context, node);
                }
            }
        }

        Ok(!self.found_instances.is_empty())
    }

    fn severity(&self) -> IssueSeverity {
        IssueSeverity::Low
    }

    fn title(&self) -> String {
        String::from("Potentially unused `private` / `internal` state variables found.")
    }

    fn description(&self) -> String {
        String::from("State variable appears to be unused. No analysis has been performed to see if any inilne assembly \
            references it. So if that's not the case, consider removing this unused variable.")
    }

    fn instances(&self) -> BTreeMap<(String, usize, String), NodeID> {
        self.found_instances.clone()
    }

    fn name(&self) -> String {
        format!("{}", IssueDetectorNamePool::UnusedStateVariable)
    }
}

#[cfg(test)]
mod unused_detector_tests {
    use serial_test::serial;

    use crate::detect::{
        detector::IssueDetector, low::unused_state_variable::UnusedStateVariablesDetector,
    };

    #[test]
    #[serial]
    fn test_unused_state_variables() {
        let context = crate::detect::test_utils::load_solidity_source_unit(
            "../tests/contract-playground/src/UnusedStateVariables.sol",
        );

        let mut detector = UnusedStateVariablesDetector::default();
        let found = detector.detect(&context).unwrap();
        // assert that the detector found an issue
        assert!(found);
        // assert that the detector found the correct number of instances
        assert_eq!(detector.instances().len(), 4);
        // assert the severity is low
        assert_eq!(
            detector.severity(),
            crate::detect::detector::IssueSeverity::Low
        );
    }
}
