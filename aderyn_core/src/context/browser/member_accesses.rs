use eyre::Result;

use crate::{
    ast::{ForStatement, MemberAccess, WhileStatement},
    visitor::ast_visitor::{ASTConstVisitor, Node},
};

pub struct MemberAccesses {
    pub member_accesses: Vec<MemberAccess>,
}

impl From<&ForStatement> for MemberAccesses {
    fn from(for_statement: &ForStatement) -> MemberAccesses {
        let mut found_member_accesses = Self {
            member_accesses: vec![],
        };
        for_statement
            .accept(&mut found_member_accesses)
            .unwrap_or_default();
        found_member_accesses
    }
}

impl From<&WhileStatement> for MemberAccesses {
    fn from(while_statement: &WhileStatement) -> MemberAccesses {
        let mut found_member_accesses = Self {
            member_accesses: vec![],
        };
        while_statement
            .accept(&mut found_member_accesses)
            .unwrap_or_default();
        found_member_accesses
    }
}

impl ASTConstVisitor for MemberAccesses {
    fn visit_member_access(&mut self, node: &MemberAccess) -> Result<bool> {
        self.member_accesses.push(node.clone());
        Ok(true)
    }
}
