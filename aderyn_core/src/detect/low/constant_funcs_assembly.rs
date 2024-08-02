use std::collections::BTreeMap;
use std::error::Error;

use crate::ast::{ASTNode, NodeID, StateMutability};

use crate::capture;
use crate::context::browser::ExtractInlineAssemblys;
use crate::context::investigator::{
    StandardInvestigationStyle, StandardInvestigator, StandardInvestigatorVisitor,
};
use crate::detect::detector::IssueDetectorNamePool;
use crate::detect::helpers;
use crate::{
    context::workspace_context::WorkspaceContext,
    detect::detector::{IssueDetector, IssueSeverity},
};
use eyre::Result;

#[derive(Default)]
pub struct ConstantFunctionContainsAssemblyDetector {
    // Keys are: [0] source file name, [1] line number, [2] character location of node.
    // Do not add items manually, use `capture!` to add nodes to this BTreeMap.
    found_instances: BTreeMap<(String, usize, String), NodeID>,
}

impl IssueDetector for ConstantFunctionContainsAssemblyDetector {
    fn detect(&mut self, context: &WorkspaceContext) -> Result<bool, Box<dyn Error>> {
        for function in helpers::get_implemented_external_and_public_functions(context) {
            if function.state_mutability() == &StateMutability::View
                || function.state_mutability() == &StateMutability::Pure
            {
                let mut tracker = AssemblyTracker {
                    has_assembly: false,
                };
                let investigator = StandardInvestigator::new(
                    context,
                    &[&(function.into())],
                    StandardInvestigationStyle::Downstream,
                )?;
                investigator.investigate(context, &mut tracker)?;

                if tracker.has_assembly {
                    capture!(self, context, function);
                }
            }
        }

        Ok(!self.found_instances.is_empty())
    }

    fn severity(&self) -> IssueSeverity {
        IssueSeverity::Low
    }

    fn title(&self) -> String {
        String::from("Functions declared `pure` / `view` but contains assembly")
    }

    fn description(&self) -> String {
        String::from("If the assembly code contains bugs or unintended side effects, it can lead to incorrect results \
            or vulnerabilities, which are hard to debug and resolve, especially when the function is meant to be simple \
            and predictable.")
    }

    fn instances(&self) -> BTreeMap<(String, usize, String), NodeID> {
        self.found_instances.clone()
    }

    fn name(&self) -> String {
        format!("{}", IssueDetectorNamePool::ConstantFunctionsAssembly)
    }
}

struct AssemblyTracker {
    has_assembly: bool,
}

impl StandardInvestigatorVisitor for AssemblyTracker {
    fn visit_any(&mut self, node: &crate::ast::ASTNode) -> eyre::Result<()> {
        // If we are already satisifed, do not bother checking
        if self.has_assembly {
            return Ok(());
        }

        if let ASTNode::FunctionDefinition(function) = node {
            // Ignore checking functions that start with `_`
            // Example - templegold contains math functions like `_rpow()`, etc that are used by view functions
            // That should be okay .. I guess? (idk ... it's open for dicussion)
            if function.name.starts_with("_") {
                return Ok(());
            }
        }

        // Check if this node has assembly code
        let assemblies = ExtractInlineAssemblys::from(node).extracted;
        if !assemblies.is_empty() {
            self.has_assembly = true;
        }
        Ok(())
    }
}

#[cfg(test)]
mod constant_functions_assembly_detector {
    use serial_test::serial;

    use crate::detect::{
        detector::IssueDetector,
        low::constant_funcs_assembly::ConstantFunctionContainsAssemblyDetector,
    };

    #[test]
    #[serial]
    fn test_constant_functions_assembly() {
        let context = crate::detect::test_utils::load_solidity_source_unit(
            "../tests/contract-playground/src/ConstantFuncsAssembly.sol",
        );

        let mut detector = ConstantFunctionContainsAssemblyDetector::default();
        let found = detector.detect(&context).unwrap();
        // assert that the detector found an issue
        assert!(found);
        // assert that the detector found the correct number of instances
        assert_eq!(detector.instances().len(), 3);
        // assert the severity is low
        assert_eq!(
            detector.severity(),
            crate::detect::detector::IssueSeverity::Low
        );
    }
}
