use crate::{
    ast::{Expression, NodeID},
    capture,
    context::workspace_context::WorkspaceContext,
    detect::detector::{IssueDetector, IssueDetectorNamePool, IssueSeverity},
};
use eyre::Result;
use std::{collections::BTreeMap, collections::HashSet, error::Error};

#[derive(Default)]
pub struct UselessErrorDetector {
    // Keys are source file name and line number
    found_instances: BTreeMap<(String, usize, String), NodeID>,
}

impl IssueDetector for UselessErrorDetector {
    fn detect(&mut self, context: &WorkspaceContext) -> Result<bool, Box<dyn Error>> {
        let error_definitions = context.error_definitions().into_iter().collect::<Vec<_>>();
        let mut used_errors = HashSet::new();

        // Collect all used errors from revert statements
        for revert_stmt in context.revert_statements() {
            //  extract the ids directly from the expression of the function call
            if let Expression::Identifier(identifier) = &*revert_stmt.error_call.expression {
                used_errors.insert(identifier.referenced_declaration);
            } else if let Expression::MemberAccess(member_access) =
                &*revert_stmt.error_call.expression
            {
                used_errors.insert(member_access.referenced_declaration.unwrap());
            }
        }

        // Identify unused errors by comparing defined and used error IDs
        for error_def in error_definitions {
            if !used_errors.contains(&error_def.id) {
                // Capture unused error instances
                capture!(self, context, error_def);
            }
        }
        Ok(!self.found_instances.is_empty())
    }

    fn title(&self) -> String {
        String::from("Unused Custom Error")
    }

    fn description(&self) -> String {
        String::from("it is recommended that the definition be removed when custom error is unused")
    }

    fn severity(&self) -> IssueSeverity {
        IssueSeverity::Low
    }

    fn instances(&self) -> BTreeMap<(String, usize, String), NodeID> {
        self.found_instances.clone()
    }

    fn name(&self) -> String {
        format!("{}", IssueDetectorNamePool::UselessError)
    }
}

#[cfg(test)]
mod useless_error_tests {
    use crate::detect::detector::IssueDetector;

    use super::UselessErrorDetector;

    #[test]
    fn test_unused_error_detection() {
        let context = crate::detect::test_utils::load_solidity_source_unit(
            "../tests/contract-playground/src/UnusedError.sol",
        );

        let mut detector = UselessErrorDetector::default();
        // Assert that the detector finds the unused error
        let found = detector.detect(&context).unwrap();
        assert!(found);
        // Assert that the detector returns the correct number of instances
        assert_eq!(detector.instances().len(), 2);
        // Assert that the detector returns the correct severity
        assert_eq!(
            detector.severity(),
            crate::detect::detector::IssueSeverity::Low
        );
        // Assert that the detector returns the correct title
        assert_eq!(detector.title(), String::from("Unused Custom Error"));
        // Assert that the detector returns the correct description
        assert_eq!(
            detector.description(),
            String::from(
                "it is recommended that the definition be removed when custom error is unused"
            )
        );
    }
}
