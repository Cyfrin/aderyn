use crate::{
    ast::{macros::accept_id, Block, UncheckedBlock},
    visitor::ast_visitor::*,
};
use eyre::Result;

impl Node for Block {
    fn accept(&self, visitor: &mut impl ASTConstVisitor) -> Result<()> {
        if visitor.visit_block(self)? {
            list_accept(&self.statements, visitor)?;
        }
        self.accept_metadata(visitor)?;
        visitor.end_visit_block(self)
    }

    fn accept_metadata(&self, visitor: &mut impl ASTConstVisitor) -> Result<()> {
        let children_ids = self.statements.iter().flat_map(|x| x.get_node_id()).collect::<Vec<_>>();
        visitor.visit_immediate_children(self.id, children_ids)?;
        Ok(())
    }

    accept_id!();
}

impl Node for UncheckedBlock {
    fn accept(&self, visitor: &mut impl ASTConstVisitor) -> Result<()> {
        if visitor.visit_unchecked_block(self)? {
            list_accept(&self.statements, visitor)?;
        }
        self.accept_metadata(visitor)?;
        visitor.end_visit_unchecked_block(self)
    }

    fn accept_metadata(&self, visitor: &mut impl ASTConstVisitor) -> Result<()> {
        let children_ids = self.statements.iter().flat_map(|x| x.get_node_id()).collect::<Vec<_>>();
        visitor.visit_immediate_children(self.id, children_ids)?;
        Ok(())
    }

    accept_id!();
}
