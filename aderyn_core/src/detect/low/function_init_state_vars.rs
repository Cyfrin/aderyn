use std::{collections::BTreeMap, error::Error};

use crate::ast::{ASTNode, Expression, FunctionCall, Identifier, NodeID};

use crate::{
    capture,
    context::{
        browser::ExtractReferencedDeclarations,
        graph::{CallGraph, CallGraphDirection, CallGraphVisitor},
        workspace_context::WorkspaceContext,
    },
    detect::detector::{IssueDetector, IssueDetectorNamePool, IssueSeverity},
};
use eyre::Result;

#[derive(Default)]
pub struct FunctionInitializingStateDetector {
    // Keys are: [0] source file name, [1] line number, [2] character location of node.
    // Do not add items manually, use `capture!` to add nodes to this BTreeMap.
    found_instances: BTreeMap<(String, usize, String), NodeID>,
}

impl IssueDetector for FunctionInitializingStateDetector {
    fn detect(&mut self, context: &WorkspaceContext) -> Result<bool, Box<dyn Error>> {
        // PLAN
        // Capture state variables that are initialized directly by calling a non constant function.
        // Go throough state variable declarations with initial value (this will be true when value
        // is set outside constructor) See if the function references non-constant state
        // variables. If it does, then capture it

        for variable_declaration in
            context.variable_declarations().into_iter().filter(|v| v.state_variable)
        {
            if let Some(Expression::FunctionCall(FunctionCall { expression, .. })) =
                variable_declaration.value.as_ref()
            {
                if let Expression::Identifier(Identifier {
                    referenced_declaration: Some(func_id),
                    ..
                }) = expression.as_ref()
                {
                    if let Some(ASTNode::FunctionDefinition(func)) = context.nodes.get(func_id) {
                        let mut tracker =
                            NonConstantStateVariableReferenceDeclarationTracker::new(context);

                        let callgraph =
                            CallGraph::new(context, &[&(func.into())], CallGraphDirection::Inward)?;

                        callgraph.accept(context, &mut tracker)?;

                        if tracker.makes_a_reference {
                            capture!(self, context, variable_declaration);
                        }
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
        String::from("Function initializing state.")
    }

    fn description(&self) -> String {
        String::from("Detects the immediate initialization of state variables through function calls that are not pure/constant, or that use \
            non-constant state variable. Remove any initialization of state variables via non-constant state variables or function calls. If variables \
            must be set upon contract deployment, locate initialization in the constructor instead.")
    }

    fn instances(&self) -> BTreeMap<(String, usize, String), NodeID> {
        self.found_instances.clone()
    }

    fn name(&self) -> String {
        format!("{}", IssueDetectorNamePool::FunctionInitializingState)
    }
}

struct NonConstantStateVariableReferenceDeclarationTracker<'a> {
    makes_a_reference: bool,
    context: &'a WorkspaceContext,
}

impl<'a> NonConstantStateVariableReferenceDeclarationTracker<'a> {
    fn new(context: &'a WorkspaceContext) -> Self {
        Self { makes_a_reference: false, context }
    }
}

impl CallGraphVisitor for NonConstantStateVariableReferenceDeclarationTracker<'_> {
    fn visit_any(&mut self, node: &ASTNode) -> eyre::Result<()> {
        // We already know the condition is satisifed
        if self.makes_a_reference {
            return Ok(());
        }

        let references = ExtractReferencedDeclarations::from(node).extracted;

        for reference in references {
            if let Some(ASTNode::VariableDeclaration(variable_declaration)) =
                self.context.nodes.get(&reference)
            {
                if variable_declaration.state_variable && !variable_declaration.constant {
                    self.makes_a_reference = true;
                }
            }
        }

        Ok(())
    }
}

#[cfg(test)]
mod function_initializing_state_tests {
    use serial_test::serial;

    use crate::detect::{
        detector::IssueDetector, low::function_init_state_vars::FunctionInitializingStateDetector,
    };

    #[test]
    #[serial]
    fn test_function_initializing_state() {
        let context = crate::detect::test_utils::load_solidity_source_unit(
            "../tests/contract-playground/src/FunctionInitializingState.sol",
        );

        let mut detector = FunctionInitializingStateDetector::default();
        let found = detector.detect(&context).unwrap();
        // assert that the detector found an issue
        assert!(found);
        // assert that the detector found the correct number of instances
        assert_eq!(detector.instances().len(), 3);
        // assert the severity is low
        assert_eq!(detector.severity(), crate::detect::detector::IssueSeverity::Low);
    }
}
