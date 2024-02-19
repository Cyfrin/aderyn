use crate::context::browser::GetImmediateChildren;
#[allow(unused_imports)]
use crate::{
    ast::{IfStatement, NodeID},
    capture,
    context::{
        browser::{
            get_children_of_node, ExtractVariableDeclarationStatements, ExtractVariableDeclarations,
        },
        workspace_context::{ASTNode, WorkspaceContext},
    },
    detect::detector::{IssueDetector, IssueDetectorNamePool, IssueSeverity},
};
use eyre::Result;
use std::{collections::BTreeMap, error::Error};

#[derive(Default)]
pub struct ChildrenDemoDetector {
    // Keys are source file name and line number
    found_instances: BTreeMap<(String, usize), NodeID>,
}

/*

In ParentChainContract.sol
The goal is to get all the variable declarations that are direct children on contract.
This should exclude variable declarations done inside function names
*/

impl IssueDetector for ChildrenDemoDetector {
    fn detect(&mut self, context: &WorkspaceContext) -> Result<bool, Box<dyn Error>> {
        /* The following code would capture lines
           {40, 41} In ParentChainContract.sol
           (This is just the outer variables}
        */
        for cd in context.contract_definitions.keys() {
            if cd.name.contains("AnotherOne") {
                for child in cd.immediate_children(context).unwrap() {
                    println!("{}", child);
                    if let ASTNode::VariableDeclaration(v) = child {
                        capture!(self, context, v);
                    }
                }
            }
        }
        /* The above prints the following
           -----------------------------
           VariableDeclaration
           VariableDeclaration
           FunctionDefinition
           FunctionDefinition
        *
        */

        /* The following code would capture lines
           {40, 41, 43, 44, 45, 51, 53} In ParentChainContract.sol
           (This is all the variables, not just the outer ones)
        */
        // for cd in context.contract_definitions.keys() {
        //     if cd.name.contains("AnotherOne") {
        //         let vars = ExtractVariableDeclarations::from(cd).extracted;
        //         for v in vars {
        //             capture!(self, context, v);
        //         }
        //     }
        // }

        Ok(!self.found_instances.is_empty())
    }

    fn severity(&self) -> IssueSeverity {
        IssueSeverity::Medium
    }

    fn title(&self) -> String {
        String::from("Demo Children Demonstration")
    }

    fn description(&self) -> String {
        String::from("Demo Children Demonstration")
    }

    fn instances(&self) -> BTreeMap<(String, usize), NodeID> {
        self.found_instances.clone()
    }

    fn name(&self) -> String {
        format!("{}", IssueDetectorNamePool::CentralizationRisk)
    }
}

#[cfg(test)]
mod children_demo_tests {
    use crate::detect::{
        detector::{detector_test_helpers::load_contract, IssueDetector},
        experimental::demo_children::ChildrenDemoDetector,
    };

    #[test]
    fn test_children_demo() {
        let context = load_contract(
            "../tests/contract-playground/out/ParentChainContract.sol/ParentChainContract.json",
        );

        let mut detector = ChildrenDemoDetector::default();
        let found = detector.detect(&context).unwrap();
        assert!(found);
        println!("{:?}", detector.instances());
        assert!(detector.instances().len() == 2);
    }
}
