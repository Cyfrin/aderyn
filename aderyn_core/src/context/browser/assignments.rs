use eyre::Result;

use crate::{
    ast::{Assignment, FunctionDefinition},
    visitor::ast_visitor::{ASTConstVisitor, Node},
};

pub struct Assignments {
    pub assignments: Vec<Assignment>,
}

impl From<&FunctionDefinition> for Assignments {
    fn from(function_definition: &FunctionDefinition) -> Assignments {
        let mut assignments = Self {
            assignments: vec![],
        };
        function_definition
            .accept(&mut assignments)
            .unwrap_or_default();
        assignments
    }
}

impl ASTConstVisitor for Assignments {
    fn visit_assignment(&mut self, node: &Assignment) -> Result<bool> {
        self.assignments.push(node.clone());
        Ok(true)
    }
}
