use std::{collections::BTreeMap, error::Error};

use crate::{
    ast::NodeID,
    capture,
    context::{
        browser::{ExtractIdentifiers, ExtractModifierInvocations, ExtractRevertStatements},
        graph::{CallGraph, CallGraphDirection, CallGraphVisitor},
        workspace_context::WorkspaceContext,
    },
    detect::{
        detector::{IssueDetector, IssueDetectorNamePool, IssueSeverity},
        helpers,
    },
};
use eyre::Result;

#[derive(Default)]
pub struct UnprotectedInitializerDetector {
    // Keys are: [0] source file name, [1] line number, [2] character location of node.
    // Do not add items manually, use `capture!` to add nodes to this BTreeMap.
    found_instances: BTreeMap<(String, usize, String), NodeID>,
}

impl IssueDetector for UnprotectedInitializerDetector {
    fn detect(&mut self, context: &WorkspaceContext) -> Result<bool, Box<dyn Error>> {
        for func in helpers::get_implemented_external_and_public_functions(context) {
            let callgraph = CallGraph::new(context, &[&func.into()], CallGraphDirection::Inward)?;
            let mut tracker = UnprotectedInitializationTracker::default();
            callgraph.accept(context, &mut tracker)?;

            if func.name.starts_with("_init") || func.name.starts_with("init") {
                if tracker.has_initializer_modifier || tracker.has_require_or_revert {
                    continue;
                }
                capture!(self, context, func);
            }
        }

        Ok(!self.found_instances.is_empty())
    }

    fn title(&self) -> String {
        String::from("Unprotected initializer")
    }

    fn description(&self) -> String {
        String::from("Consider protecting the initializer functions with modifiers.")
    }

    fn severity(&self) -> IssueSeverity {
        IssueSeverity::High
    }

    fn instances(&self) -> BTreeMap<(String, usize, String), NodeID> {
        self.found_instances.clone()
    }

    fn name(&self) -> String {
        format!("{}", IssueDetectorNamePool::UnprotectedInitializer)
    }
}

#[derive(Default, Debug)]
struct UnprotectedInitializationTracker {
    has_require_or_revert: bool,
    has_initializer_modifier: bool, // devtooligan's suggestion
}

impl CallGraphVisitor for UnprotectedInitializationTracker {
    fn visit_any(&mut self, node: &crate::ast::ASTNode) -> eyre::Result<()> {
        // Check for revert(), require(), revert SomeError()
        let has_req_or_revert_calls = ExtractIdentifiers::from(node)
            .extracted
            .into_iter()
            .any(|x| x.name == "require" || x.name == "revert");

        let has_revert_stmnts = !ExtractRevertStatements::from(node).extracted.is_empty();

        if has_req_or_revert_calls || has_revert_stmnts {
            self.has_require_or_revert = true;
        }

        // Check if modifier name is "initialized" and assume it works
        // This is done becauase often times initialized comes from openzeppelin and it is out of
        // scope when running aderyn due to it being a library

        let modifier_invocations = ExtractModifierInvocations::from(node).extracted;

        for inv in modifier_invocations {
            match inv.modifier_name {
                crate::ast::IdentifierOrIdentifierPath::Identifier(n) => {
                    if n.name == "initializer" {
                        self.has_initializer_modifier = true;
                    }
                }
                crate::ast::IdentifierOrIdentifierPath::IdentifierPath(n) => {
                    if n.name == "initializer" {
                        self.has_initializer_modifier = true;
                    }
                }
            }
        }

        Ok(())
    }
}

#[cfg(test)]
mod unprotected_initializer {
    use serial_test::serial;

    use crate::detect::detector::IssueDetector;

    use super::UnprotectedInitializerDetector;

    #[test]
    #[serial]
    fn test_unprotected_initializer_by_loading_contract_directly() {
        let context = crate::detect::test_utils::load_solidity_source_unit(
            "../tests/contract-playground/src/UnprotectedInitialize.sol",
        );

        let mut detector = UnprotectedInitializerDetector::default();
        let found = detector.detect(&context).unwrap(); // assert that the detector found an abi encode packed
        assert!(found);
        // println!("{:?}", detector.instances());
        assert_eq!(detector.instances().len(), 1);
    }
}
