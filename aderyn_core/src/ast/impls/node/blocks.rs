use crate::{ast::Block, visitor::ast_visitor::*};
use eyre::Result;
use std::fmt::Display;

impl Node for Block {
    fn accept(&self, visitor: &mut impl ASTConstVisitor) -> Result<()> {
        if visitor.visit_block(self)? {
            list_accept(&self.statements, visitor)?;
        }
        self.accept_metadata(visitor)?;
        visitor.end_visit_block(self)
    }

    fn accept_metadata(&self, visitor: &mut impl ASTConstVisitor) -> Result<()> {
        let children_ids = self
            .statements
            .iter()
            .flat_map(|x| x.get_node_id())
            .collect::<Vec<_>>();
        visitor.visit_immediate_children(self.id, children_ids)?;
        Ok(())
    }

    fn accept_id(&self, visitor: &mut impl ASTConstVisitor) -> Result<()> {
        visitor.visit_node_id(Some(self.id))?;
        Ok(())
    }
}
