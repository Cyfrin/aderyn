use std::{collections::BTreeMap, error::Error};

use crate::ast::{FunctionKind, NodeID};

use crate::{
    capture,
    context::{browser::ExtractVariableDeclarations, workspace::WorkspaceContext},
    detect::detector::{IssueDetector, IssueDetectorNamePool, IssueSeverity},
};
use eyre::Result;

#[derive(Default)]
pub struct FunctionPointerInConstructorDetector {
    // Keys are: [0] source file name, [1] line number, [2] character location of node.
    // Do not add items manually, use `capture!` to add nodes to this BTreeMap.
    found_instances: BTreeMap<(String, usize, String), NodeID>,
}

impl IssueDetector for FunctionPointerInConstructorDetector {
    fn detect(&mut self, context: &WorkspaceContext) -> Result<bool, Box<dyn Error>> {
        // PLAN:
        // Catch all the function pointers in constructors that compile below 0.5.9

        for func in context
            .function_definitions()
            .into_iter()
            .filter(|f| f.kind() == &FunctionKind::Constructor)
            .filter(|f| f.compiles_for_solc_below_0_5_9(context))
        {
            let variable_declarations = ExtractVariableDeclarations::from(func).extracted;

            for variable_declaration in variable_declarations {
                if variable_declaration
                    .type_descriptions
                    .type_string
                    .as_ref()
                    .is_some_and(|type_string| type_string.starts_with("function "))
                {
                    capture!(self, context, variable_declaration);
                }
            }
        }

        Ok(!self.found_instances.is_empty())
    }

    fn severity(&self) -> IssueSeverity {
        IssueSeverity::Low
    }

    fn title(&self) -> String {
        String::from("Function Pointer in Constructor")
    }

    fn description(&self) -> String {
        String::from("solc versions below 0.5.9 contain a compiler bug leading to unexpected behavior when calling uninitialized function pointers in constructors. It is recommended to not use function pointers in constructors.")
    }

    fn instances(&self) -> BTreeMap<(String, usize, String), NodeID> {
        self.found_instances.clone()
    }

    fn name(&self) -> String {
        format!("{}", IssueDetectorNamePool::FunctionPointerInConstructor)
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
        pub fn compiles_for_solc_below_0_5_9(&self, context: &WorkspaceContext) -> bool {
            if let Some(source_unit) = self.closest_ancestor_of_type(context, NodeType::SourceUnit)
            {
                let pragma_directives = ExtractPragmaDirectives::from(source_unit).extracted;

                if let Some(pragma_directive) = pragma_directives.first() {
                    if let Ok(pragma_semver) = helpers::pragma_directive_to_semver(pragma_directive)
                    {
                        if version_req_allows_below_0_5_9(&pragma_semver) {
                            return true;
                        }
                    }
                }
            }
            false
        }
        pub fn compiles_for_solc_below_0_6_5(&self, context: &WorkspaceContext) -> bool {
            if let Some(source_unit) = self.closest_ancestor_of_type(context, NodeType::SourceUnit)
            {
                let pragma_directives = ExtractPragmaDirectives::from(source_unit).extracted;

                if let Some(pragma_directive) = pragma_directives.first() {
                    if let Ok(pragma_semver) = helpers::pragma_directive_to_semver(pragma_directive)
                    {
                        if version_req_allows_below_0_6_5(&pragma_semver) {
                            return true;
                        }
                    }
                }
            }
            false
        }
    }

    fn version_req_allows_below_0_5_9(version_req: &VersionReq) -> bool {
        // If it matches any 0.4.0 to 0.4.26, return true
        for i in 0..=26 {
            let version = Version::from_str(&format!("0.4.{}", i)).unwrap();
            if version_req.matches(&version) {
                return true;
            }
        }

        // If it matches any 0.5.0 to 0.5.8, return true
        for i in 0..=8 {
            let version = Version::from_str(&format!("0.5.{}", i)).unwrap();
            if version_req.matches(&version) {
                return true;
            }
        }

        // Else, return false
        false
    }
    fn version_req_allows_below_0_6_5(version_req: &VersionReq) -> bool {
        // If it matches any 0.4.0 to 0.4.26, return true
        for i in 0..=26 {
            let version = Version::from_str(&format!("0.4.{}", i)).unwrap();
            if version_req.matches(&version) {
                return true;
            }
        }

        // If it matches any 0.5.0 to 0.5.17, return true
        for i in 0..=17 {
            let version = Version::from_str(&format!("0.5.{}", i)).unwrap();
            if version_req.matches(&version) {
                return true;
            }
        }

        // If it matches any 0.6.0 to 0.6.4, return true
        for i in 0..=4 {
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
mod function_pointers_tests {

    use crate::detect::{
        detector::IssueDetector,
        low::function_pointer_in_constructor::FunctionPointerInConstructorDetector,
    };

    #[test]

    fn test_function_pointers() {
        let context = crate::detect::test_utils::load_solidity_source_unit(
            "../tests/contract-playground/src/FunctionPointers.sol",
        );

        let mut detector = FunctionPointerInConstructorDetector::default();
        let found = detector.detect(&context).unwrap();
        assert!(found);
        assert_eq!(detector.instances().len(), 1);
    }
}
