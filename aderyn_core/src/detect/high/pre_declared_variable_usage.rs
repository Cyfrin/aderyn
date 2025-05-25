use std::{
    collections::{BTreeMap, HashSet},
    error::Error,
};

use crate::ast::{ASTNode, NodeID};

use crate::{
    capture,
    context::{
        browser::{ExtractIdentifiers, ExtractVariableDeclarations},
        workspace::WorkspaceContext,
    },
    detect::{
        detector::{IssueDetector, IssueDetectorNamePool, IssueSeverity},
        helpers,
    },
};
use eyre::Result;

// HOW TO USE THIS TEMPLATE:
// 1. Copy this file and rename it to the snake_case version of the issue you are detecting.
// 2. Rename the PreDeclaredLocalVariableUsageDetector struct and impl to your new issue name.
// 3. Add this file and detector struct to the mod.rs file in the same directory.
// 4. Implement the detect function to find instances of the issue.

#[derive(Default)]
pub struct PreDeclaredLocalVariableUsageDetector {
    // Keys are: [0] source file name, [1] line number, [2] character location of node.
    // Do not add items manually, use `capture!` to add nodes to this BTreeMap.
    found_instances: BTreeMap<(String, usize, String), NodeID>,
}

impl IssueDetector for PreDeclaredLocalVariableUsageDetector {
    fn detect(&mut self, context: &WorkspaceContext) -> Result<bool, Box<dyn Error>> {
        // Since this is restricted to local variables, we examine each function independently
        for function in context.function_definitions().into_iter().filter(|&f| f.implemented) {
            let local_variable_declaration_ids = ExtractVariableDeclarations::from(function)
                .extracted
                .iter()
                .map(|vd| vd.id)
                .collect::<HashSet<_>>();

            let used_local_variables = ExtractIdentifiers::from(function).extracted;

            let used_local_variables = used_local_variables
                .iter()
                .filter(|identifier| {
                    identifier.referenced_declaration.is_some_and(|referenced_declaration| {
                        local_variable_declaration_ids.contains(&referenced_declaration)
                    })
                })
                .collect::<HashSet<_>>();

            for used in used_local_variables {
                if let Some(id) = used.referenced_declaration {
                    if let Some(ASTNode::VariableDeclaration(variable_declaration)) =
                        context.nodes.get(&id)
                    {
                        let used_offset = helpers::get_node_offset(&used.into());
                        let declaration_offset =
                            helpers::get_node_offset(&variable_declaration.into());

                        if let (Some(used_offset), Some(declaration_offset)) =
                            (used_offset, declaration_offset)
                        {
                            if used_offset < declaration_offset {
                                capture!(self, context, used);
                            }
                        }
                    }
                }
            }
        }

        Ok(!self.found_instances.is_empty())
    }

    fn severity(&self) -> IssueSeverity {
        IssueSeverity::High
    }

    fn title(&self) -> String {
        String::from("Usage of variable before declaration")
    }

    fn description(&self) -> String {
        String::from("Declare the variable before using it to avoid unintended consequences.")
    }

    fn instances(&self) -> BTreeMap<(String, usize, String), NodeID> {
        self.found_instances.clone()
    }

    fn name(&self) -> String {
        IssueDetectorNamePool::PreDeclaredLocalVariableUsage.to_string()
    }
}

#[cfg(test)]
mod pre_declared_variable_usage_tests {

    use crate::detect::{
        detector::IssueDetector,
        high::pre_declared_variable_usage::PreDeclaredLocalVariableUsageDetector,
    };

    #[test]

    fn test_pre_declared_variable_usage() {
        let context = crate::detect::test_utils::load_solidity_source_unit(
            "../tests/contract-playground/src/PreDeclaredVarUsage.sol",
        );

        let mut detector = PreDeclaredLocalVariableUsageDetector::default();
        let found = detector.detect(&context).unwrap();
        assert!(found);
        assert_eq!(detector.instances().len(), 1);
    }
}
