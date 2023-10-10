use std::error::Error;

use crate::{ast::MemberAccess, visitor::ast_visitor::ASTConstVisitor, loader::loader::{ContractLoader, ASTNode}, detector::detector::{Detector, IssueSeverity}};
use eyre::Result;
use crate::visitor::ast_visitor::Node;


#[derive(Default)]
pub struct DelegateCallInLoopDetector {
    pub found_delegate_call_in_loop: Vec<Option<ASTNode>>,
}

impl ASTConstVisitor for DelegateCallInLoopDetector {
    fn visit_member_access(&mut self, node: &MemberAccess) -> Result<bool> {
        if node.member_name == "delegatecall" {
            self.found_delegate_call_in_loop.push(Some(ASTNode::MemberAccess(node.clone())));
        }
        Ok(true)
    }
}


impl Detector for DelegateCallInLoopDetector {
    fn detect(&mut self, loader: &ContractLoader) -> Result<bool, Box<dyn Error>> {
        for for_statement in loader.get_for_statements() {
            for_statement.accept(self)?;
        }
        for while_statement in loader.get_while_statements() {
            while_statement.accept(self)?;
        }

        println!("Found {} delegatecalls in loops", self.found_delegate_call_in_loop.len());
        Ok(self.found_delegate_call_in_loop.len() > 0)
    }

    fn severity(&self) -> IssueSeverity {
        IssueSeverity::High
    }

    fn title(&self) -> String {
        String::from("Using `delegatecall` in loop")
    }

    fn description(&self) -> String {
        String::from("When calling `delegatecall` the same `msg.value` amount will be accredited multiple times.")
    }

    fn instances(&self) -> Vec<Option<ASTNode>> {
        self.found_delegate_call_in_loop.clone()
    }  
}