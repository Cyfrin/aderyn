use std::error::Error;

use crate::{ast::MemberAccess, visitor::ast_visitor::ASTConstVisitor, loader::loader::ContractLoader};
use eyre::Result;
use crate::visitor::ast_visitor::Node;
use super::detector::Detector;

#[derive(Default)]
pub struct DelegateCallInLoopDetector {
    pub found_delegate_call_in_loop: Vec<MemberAccess>,
}

impl ASTConstVisitor for DelegateCallInLoopDetector {
    fn visit_member_access(&mut self, node: &MemberAccess) -> Result<bool> {
        if node.member_name == "delegatecall" {
            self.found_delegate_call_in_loop.push(node.clone());
        }
        Ok(true)
    }
}


impl Detector for DelegateCallInLoopDetector {
    fn detect(&mut self, loader: &ContractLoader) -> Result<(), Box<dyn Error>> {
        for for_statement in loader.get_for_statements() {
            for_statement.accept(self)?;
        }
        for while_statement in loader.get_while_statements() {
            while_statement.accept(self)?;
        }
        println!("Found delegatecall in loop: {:?}", self.found_delegate_call_in_loop);
        Ok(())
    }
}