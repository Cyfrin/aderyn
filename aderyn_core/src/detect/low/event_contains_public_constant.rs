use crate::ast::{Expression, FunctionKind, Identifier, Mutability, NodeID, NodeType, Visibility};
use crate::capture;
use crate::context::browser::GetClosestAncestorOfTypeX;
use crate::context::workspace_context::ASTNode;
use crate::detect::detector::IssueDetectorNamePool;
use crate::{
    context::workspace_context::WorkspaceContext,
    detect::detector::{IssueDetector, IssueSeverity},
};
use eyre::Result;
use std::collections::BTreeMap;
use std::convert::identity;
use std::error::Error;

#[derive(Default)]
pub struct EventContainsPublicConstantDetector {
    // Keys are: [0] source file name, [1] line number, [2] character location of node.
    // Do not add items manually, use `capture!` to add nodes to this BTreeMap.
    found_instances: BTreeMap<(String, usize, String), NodeID>,
    hints: BTreeMap<(String, usize, String), String>,
}

impl IssueDetector for EventContainsPublicConstantDetector {
    fn detect(&mut self, context: &WorkspaceContext) -> Result<bool, Box<dyn Error>> {
        // When you have found an instance of the issue,
        // use the following macro to add it to `found_instances`:
        //
        // capture!(self, context, item);
        // capture!(self, context, item, "hint");

        for emit_statement in context.emit_statements() {
            let Some(ASTNode::FunctionDefinition(function_definition)) =
                emit_statement.closest_ancestor_of_type(context, NodeType::FunctionDefinition)
            else {
                continue;
            };
            // It's okay to emit public constant from a constructor
            if function_definition.kind() == &FunctionKind::Constructor {
                continue;
            }
            if emit_statement
                .event_call
                .has_constructor_upstream(context)
                .is_some_and(identity)
            {
                continue;
            }
            if emit_statement.event_call.arguments.iter().any(|argument| {
                if let Expression::Identifier(Identifier {
                    referenced_declaration: Some(id),
                    ..
                }) = argument
                {
                    if let Some(ASTNode::VariableDeclaration(v)) = context.nodes.get(id) {
                        if v.state_variable
                            && v.mutability() == Some(&Mutability::Constant)
                            && v.visibility == Visibility::Public
                        {
                            return true;
                        }
                    }
                }
                false
            }) {
                capture!(self, context, emit_statement.event_call);
            }
        }

        Ok(!self.found_instances.is_empty())
    }

    fn severity(&self) -> IssueSeverity {
        IssueSeverity::Low
    }

    fn title(&self) -> String {
        String::from("Emit statement contains public constant.")
    }

    fn description(&self) -> String {
        String::from("Public constant is being emitted. This is not required because the value doesn't change after deployment.")
    }

    fn instances(&self) -> BTreeMap<(String, usize, String), NodeID> {
        self.found_instances.clone()
    }

    fn hints(&self) -> BTreeMap<(String, usize, String), String> {
        self.hints.clone()
    }

    fn name(&self) -> String {
        IssueDetectorNamePool::EventContainsPublicConstant.to_string()
    }
}

pub mod function_call_information_helper {

    use crate::{
        ast::{FunctionCall, FunctionKind},
        context::{
            graph::{CallGraph, CallGraphDirection, CallGraphVisitor},
            workspace_context::WorkspaceContext,
        },
    };

    impl FunctionCall {
        pub fn has_constructor_upstream(&self, context: &WorkspaceContext) -> Option<bool> {
            let callgraph =
                CallGraph::new(context, &[&self.into()], CallGraphDirection::Outward).ok()?;
            let mut constructor_tracker = ConstructorTracker {
                touches_constructor: false,
            };
            callgraph.accept(context, &mut constructor_tracker).ok()?;
            Some(constructor_tracker.touches_constructor)
        }
    }

    struct ConstructorTracker {
        touches_constructor: bool,
    }

    impl CallGraphVisitor for ConstructorTracker {
        fn visit_outward_function_definition(
            &mut self,
            node: &crate::ast::FunctionDefinition,
        ) -> eyre::Result<()> {
            if self.touches_constructor {
                return Ok(());
            }
            if node.kind() == &FunctionKind::Constructor {
                self.touches_constructor = true;
            }
            Ok(())
        }
    }
}

#[cfg(test)]
mod event_contains_public_constant_detector_tests {
    use serial_test::serial;

    use crate::detect::{
        detector::IssueDetector,
        low::event_contains_public_constant::EventContainsPublicConstantDetector,
    };

    #[test]
    #[serial]
    fn test_event_contains_public_constant() {
        let context = crate::detect::test_utils::load_solidity_source_unit(
            "../tests/contract-playground/src/EventContainsPublicConstant.sol",
        );

        let mut detector = EventContainsPublicConstantDetector::default();
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
    }
}
