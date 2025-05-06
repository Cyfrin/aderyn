#![allow(clippy::collapsible_match)]
use std::{collections::BTreeMap, error::Error};

use aderyn_core::{
    ast::NodeID,
    capture,
    context::{
        browser::{AppearsAfterNodeLocation, AppearsBeforeNodeLocation, GetImmediateParent},
        workspace::{ASTNode, WorkspaceContext},
    },
    detect::detector::{IssueDetector, IssueDetectorNamePool, IssueSeverity},
};
use eyre::Result;

#[derive(Default)]
pub struct ImmediateParentDemonstrator {
    // Keys are: [0] source file name, [1] line number, [2] character location of node.
    // Do not add items manually, use `capture!` to add nodes to this BTreeMap.
    found_instances: BTreeMap<(String, usize, String), NodeID>,
}

/*

In ParentChainContract.sol, there is only 1 assignment done. The goal is to capture it first, second and third parent
*/

impl IssueDetector for ImmediateParentDemonstrator {
    fn detect(&mut self, context: &WorkspaceContext) -> Result<bool, Box<dyn Error>> {
        for assignment in context.assignments() {
            println!("0 {}", assignment);
            capture!(self, context, assignment);
            if let Some(first_parent) = assignment.parent(context) {
                if let ASTNode::ExpressionStatement(expr_stmnt) = first_parent {
                    println!("1 {}", expr_stmnt);
                    if let Some(second_parent) = first_parent.parent(context) {
                        if let ASTNode::Block(for_statement) = second_parent {
                            println!("2 {}", for_statement);
                            capture!(self, context, second_parent);
                            if let Some(third_parent) = for_statement.parent(context) {
                                if let ASTNode::ForStatement(block) = third_parent {
                                    println!("3 {}", block);
                                    capture!(self, context, third_parent);
                                }

                                assert!(first_parent
                                    .appears_after(context, second_parent)
                                    .unwrap());
                                assert!(first_parent
                                    .appears_after(context, for_statement)
                                    .unwrap());
                                assert!(expr_stmnt.appears_after(context, for_statement).unwrap());
                                assert!(second_parent
                                    .appears_after(context, third_parent)
                                    .unwrap());
                                assert!(second_parent
                                    .appears_before(context, first_parent)
                                    .unwrap());
                                assert!(third_parent
                                    .appears_before(context, second_parent)
                                    .unwrap());
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
        String::from("ImmediateParentDemonstrator")
    }

    fn description(&self) -> String {
        String::from("ImmediateParentDemonstrator")
    }

    fn instances(&self) -> BTreeMap<(String, usize, String), NodeID> {
        self.found_instances.clone()
    }

    fn name(&self) -> String {
        format!("{}", IssueDetectorNamePool::CentralizationRisk)
    }
}
