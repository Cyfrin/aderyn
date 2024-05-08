use std::{collections::BTreeMap, error::Error};

use crate::{
    ast::{Expression, NodeID},
    capture,
    context::{
        browser::{
            ExtractMemberAccesses,
            GetImmediateParent, // Usa el re-export público aquí
        },
        workspace_context::{ASTNode, WorkspaceContext},
    },
    detect::detector::{IssueDetector, IssueDetectorNamePool, IssueSeverity},
};
use eyre::Result;

// HOW TO USE THIS TEMPLATE:
// 1. Copy this file and rename it to the snake_case version of the issue you are detecting.
// 2. Rename the TemplateDetector struct and impl to your new issue name.
// 3. Add this file and detector struct to the mod.rs file in the same directory.
// 4. Implement the detect function to find instances of the issue.

#[derive(Default)]
pub struct TxOriginAccesControl {
    // Keys are source file name and line number
    found_instances: BTreeMap<(String, usize, String), NodeID>,
}

impl IssueDetector for TxOriginAccesControl {
    fn detect(&mut self, context: &WorkspaceContext) -> Result<bool, Box<dyn Error>> {
        // When you have found an instance of the issue,
        // use the following macro to add it to `found_instances`:
        //
        // capture!(self, context, item);

        let requires = context
            .identifiers()
            .into_iter()
            .filter(|&id| id.name == "require");

        for require in requires {
            if let Some(ASTNode::FunctionCall(fc)) = require.parent(context) {
                let member_accesses = ExtractMemberAccesses::from(fc).extracted;
                for member_access in &member_accesses {
                    if member_access.member_name == "origin" {
                        if let Expression::Identifier(identifier) =
                            member_access.expression.as_ref()
                        {
                            if identifier.name == "tx" {
                                capture!(self, context, member_access);
                            }
                        }
                    }
                }
            }
        }

        Ok(!self.found_instances.is_empty())
    }
    // if let Some(ASTNode::MemberAccess(member_name)) = context
    //     .member_accesses()
    //     .closest_ancestor_of_type(context, NodeType::MemberAccess)
    // {
    //     let import_directives = member_name.import_directives();

    //     if import_directives.iter().any(|directive| {
    //         directive
    //             .absolute_path
    //             .as_ref()
    //             .map_or(false, |path| path.contains("MemberAccess"))
    //     }) && context.member_accesses().member_name == "MemberAccess"
    //     {
    //     }

    //     if let Expression::BinaryOperation(left_op) = op.left_expression.as_ref() {
    //         if left_op.operator == "tx" {
    //             capture!(self, context, left_op)
    //         }
    //     }
    // }
    //     }

    //     capture!(self, context, id);
    // }

    fn severity(&self) -> IssueSeverity {
        IssueSeverity::Low
    }

    fn title(&self) -> String {
        String::from("Tx.origin access control")
    }

    fn description(&self) -> String {
        String::from("Malicious contracts can impersonate users to perform actions relying on the tx.origin.")
    }

    fn instances(&self) -> BTreeMap<(String, usize, String), NodeID> {
        self.found_instances.clone()
    }

    fn name(&self) -> String {
        format!("{}", IssueDetectorNamePool::TxOriginAccesControl)
    }
}

// #[cfg(test)]
// mod template_detector_tests {
//     use crate::detect::{
//         detector::{detector_test_helpers::load_contract, IssueDetector},
//         low::template_detector::TxOriginAccesControl,
//     };

//     #[test]
//     fn test_template_detector() {
//         let context = load_contract(
//             "../tests/contract-playground/out/ArbitraryTransferFrom.sol/ArbitraryTransferFrom.json",
//         );

//         // let mut detector = TxOriginAccesControl::default();
//         // let found = detector.detect(&context).unwrap();
//         // // assert that the detector found an issue
//         // assert!(found);
//         // // assert that the detector found the correct number of instances
//         // assert_eq!(detector.instances().len(), 1);
//         // // assert the severity is low
//         // assert_eq!(
//         //     detector.severity(),
//         //     crate::detect::detector::IssueSeverity::Low
//         // );
//         // // assert the title is correct
//         // assert_eq!(detector.title(), String::from("Low Issue Title"));
//         // // assert the description is correct
//         // assert_eq!(
//         //     detector.description(),
//         //     String::from("Description of the low issue.")
//         // );
//     }
// }
