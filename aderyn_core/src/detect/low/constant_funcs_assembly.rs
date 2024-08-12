use std::collections::BTreeMap;
use std::error::Error;
use std::str::FromStr;

use crate::ast::{ASTNode, NodeID, NodeType, StateMutability};

use crate::capture;
use crate::context::browser::{
    ExtractInlineAssemblys, ExtractPragmaDirectives, GetClosestAncestorOfTypeX,
};
use crate::context::investigator::{
    StandardInvestigationStyle, StandardInvestigator, StandardInvestigatorVisitor,
};
use crate::detect::detector::IssueDetectorNamePool;
use crate::detect::helpers::{self, pragma_directive_to_semver};
use crate::{
    context::workspace_context::WorkspaceContext,
    detect::detector::{IssueDetector, IssueSeverity},
};
use eyre::Result;
use semver::{Version, VersionReq};

#[derive(Default)]
pub struct ConstantFunctionContainsAssemblyDetector {
    // Keys are: [0] source file name, [1] line number, [2] character location of node.
    // Do not add items manually, use `capture!` to add nodes to this BTreeMap.
    found_instances: BTreeMap<(String, usize, String), NodeID>,
}

impl IssueDetector for ConstantFunctionContainsAssemblyDetector {
    fn detect(&mut self, context: &WorkspaceContext) -> Result<bool, Box<dyn Error>> {
        for function in helpers::get_implemented_external_and_public_functions(context) {
            // First, check the eligibility for this function by checking
            if let Some(ASTNode::SourceUnit(source_unit)) =
                function.closest_ancestor_of_type(context, NodeType::SourceUnit)
            {
                // Store the extracted directives in a variable to extend its lifetime
                let extracted_directives = ExtractPragmaDirectives::from(source_unit).extracted;
                let pragma_directive = extracted_directives.first();

                if let Some(pragma_directive) = pragma_directive {
                    let version_req = pragma_directive_to_semver(pragma_directive);
                    if let Ok(version_req) = version_req {
                        if version_req_allows_below_0_5_0(&version_req) {
                            // Only run the logic if pragma is allowed to run on solc <0.5.0

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

fn version_req_allows_below_0_5_0(version_req: &VersionReq) -> bool {
    // If it matches any 0.4.0 to 0.4.26, return true
    for i in 0..=26 {
        let version: semver::Version = Version::from_str(&format!("0.4.{}", i)).unwrap();
        if version_req.matches(&version) {
            return true;
        }
    }

    // Else, return false
    false
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
            if function.name.starts_with('_') {
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
