use std::{collections::BTreeMap, error::Error};

use crate::ast::{NodeID, StateMutability};

use crate::{
    capture,
    context::{
        browser::ApproximateStorageChangeFinder,
        graph::{CallGraphConsumer, CallGraphDirection, CallGraphVisitor},
        workspace::WorkspaceContext,
    },
    detect::{
        detector::{IssueDetector, IssueDetectorNamePool, IssueSeverity},
        helpers,
    },
};
use eyre::Result;

#[derive(Default)]
pub struct ConstantFunctionChangesStateDetector {
    // Keys are: [0] source file name, [1] line number, [2] character location of node.
    // Do not add items manually, use `capture!` to add nodes to this BTreeMap.
    found_instances: BTreeMap<(String, usize, String), NodeID>,
}

impl IssueDetector for ConstantFunctionChangesStateDetector {
    fn detect(&mut self, context: &WorkspaceContext) -> Result<bool, Box<dyn Error>> {
        for func in helpers::get_implemented_external_and_public_functions(context) {
            // Rule applies to only view functions, so ignore the rest
            if func.state_mutability() != &StateMutability::View {
                continue;
            }
            // Check if this func is compilable for solc < 0.5.0. If not, move on to the next
            if !func.compiles_for_solc_below_0_5_0(context) {
                continue;
            }
            // Now, investigate the function to see if there is scope for any state variable changes
            let mut tracker = StateVariableChangeTracker { state_var_has_changed: false, context };

            // Keep legacy for this because it is for solc version beloe 0.5.0 and the function
            // selectors don't exist
            let callgraph = CallGraphConsumer::get_legacy(
                context,
                &[&(func.into())],
                CallGraphDirection::Inward,
            )?;
            callgraph.accept(context, &mut tracker)?;

            if tracker.state_var_has_changed {
                capture!(self, context, func);
            }
        }

        Ok(!self.found_instances.is_empty())
    }

    fn severity(&self) -> IssueSeverity {
        IssueSeverity::High
    }

    fn title(&self) -> String {
        String::from("Constant functions changes state")
    }

    fn description(&self) -> String {
        String::from("Function is declared constant/view but it changes state. Ensure that the attributes of contract compiled prior to 0.5 are correct.")
    }

    fn instances(&self) -> BTreeMap<(String, usize, String), NodeID> {
        self.found_instances.clone()
    }

    fn name(&self) -> String {
        IssueDetectorNamePool::ConstantFunctionChangesState.to_string()
    }
}

struct StateVariableChangeTracker<'a> {
    state_var_has_changed: bool,
    context: &'a WorkspaceContext,
}

impl CallGraphVisitor for StateVariableChangeTracker<'_> {
    fn visit_any(&mut self, node: &crate::ast::ASTNode) -> eyre::Result<()> {
        if self.state_var_has_changed {
            return Ok(());
        }
        // Check for state variable changes
        let finder = ApproximateStorageChangeFinder::from(self.context, node);
        if finder.state_variables_have_been_manipulated() {
            self.state_var_has_changed = true;
        }
        Ok(())
    }
}

mod func_compilation_solc_pragma_helper {
    use std::str::FromStr;

    use semver::{Version, VersionReq};

    use crate::{
        ast::{FunctionDefinition, NodeType},
        context::{
            browser::{ExtractPragmaDirectives, GetClosestAncestorOfTypeX},
            workspace::WorkspaceContext,
        },
        detect::helpers,
    };

    impl FunctionDefinition {
        pub fn compiles_for_solc_below_0_5_0(&self, context: &WorkspaceContext) -> bool {
            if let Some(source_unit) = self.closest_ancestor_of_type(context, NodeType::SourceUnit)
            {
                let pragma_directives = ExtractPragmaDirectives::from(source_unit).extracted;

                if let Some(pragma_directive) = pragma_directives.first() {
                    if let Ok(pragma_semver) = helpers::pragma_directive_to_semver(pragma_directive)
                    {
                        if version_req_allows_below_0_5_0(&pragma_semver) {
                            return true;
                        }
                    }
                }
            }
            false
        }
    }

    fn version_req_allows_below_0_5_0(version_req: &VersionReq) -> bool {
        // If it matches any 0.4.0 to 0.4.26, return true
        for i in 0..=26 {
            let version = Version::from_str(&format!("0.4.{}", i)).unwrap();
            if version_req.matches(&version) {
                return true;
            }
        }

        // Else, return false
        false
    }
}

#[cfg(test)]
mod constant_func_changing_state {

    use crate::detect::{
        detector::IssueDetector,
        high::const_func_changes_state::ConstantFunctionChangesStateDetector,
    };

    #[test]

    fn test_constant_function_changing_state() {
        let context = crate::detect::test_utils::load_solidity_source_unit(
            "../tests/contract-playground/src/ConstFuncChangeState.sol",
        );

        let mut detector = ConstantFunctionChangesStateDetector::default();
        let found = detector.detect(&context).unwrap();
        assert!(found);
        assert_eq!(detector.instances().len(), 1);
    }
}
