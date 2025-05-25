use std::{collections::BTreeMap, error::Error, str::FromStr};

use crate::ast::{ASTNode, NodeID, NodeType, StateMutability};

use crate::{
    capture,
    context::browser::{
        ExtractInlineAssemblies, ExtractPragmaDirectives, GetClosestAncestorOfTypeX,
    },
};

use crate::{
    context::{
        graph::{CallGraphConsumer, CallGraphDirection, CallGraphVisitor},
        workspace::WorkspaceContext,
    },
    detect::{
        detector::{IssueDetector, IssueDetectorNamePool, IssueSeverity},
        helpers::{self, pragma_directive_to_semver},
    },
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
                                let mut tracker = AssemblyTracker { has_assembly: false };
                                // keep legacy because < 0.5.0
                                let callgraph = CallGraphConsumer::get_legacy(
                                    context,
                                    &[&(function.into())],
                                    CallGraphDirection::Inward,
                                )?;
                                callgraph.accept(context, &mut tracker)?;

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
        String::from("Constant Function Contains Assembly")
    }

    fn description(&self) -> String {
        String::from("constant/pure/view was not enforced prior to Solidity 0.5. Starting from Solidity 0.5, a call to a constant/pure/view function uses the STATICCALL opcode, \
        which reverts in case of state modification. As a result, a call to an incorrectly labeled function may trap a contract compiled with Solidity 0.5. \
        https://docs.soliditylang.org/en/develop/050-breaking-changes.html#interoperability-with-older-contracts")
    }

    fn instances(&self) -> BTreeMap<(String, usize, String), NodeID> {
        self.found_instances.clone()
    }

    fn name(&self) -> String {
        format!("{}", IssueDetectorNamePool::ConstantFunctionContainsAssembly)
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

impl CallGraphVisitor for AssemblyTracker {
    fn visit_any(&mut self, node: &crate::ast::ASTNode) -> eyre::Result<()> {
        // If we are already satisfied, do not bother checking
        if self.has_assembly {
            return Ok(());
        }

        if let ASTNode::FunctionDefinition(function) = node {
            // Ignore checking functions that start with `_`
            // Example - templegold contains math functions like `_rpow()`, etc that are used by
            // view functions That should be okay .. I guess? (idk ... it's open for
            // discussion)
            if function.name.starts_with('_') {
                return Ok(());
            }
        }

        // Check if this node has assembly code
        let assemblies = ExtractInlineAssemblies::from(node).extracted;
        if !assemblies.is_empty() {
            self.has_assembly = true;
        }
        Ok(())
    }
}

#[cfg(test)]
mod constant_functions_assembly_detector {

    use crate::detect::{
        detector::IssueDetector,
        low::constant_function_contains_assembly::ConstantFunctionContainsAssemblyDetector,
    };

    #[test]

    fn test_constant_functions_assembly() {
        let context = crate::detect::test_utils::load_solidity_source_unit(
            "../tests/contract-playground/src/ConstantFuncsAssembly.sol",
        );

        let mut detector = ConstantFunctionContainsAssemblyDetector::default();
        let found = detector.detect(&context).unwrap();
        assert!(found);
        assert_eq!(detector.instances().len(), 3);
    }
}
