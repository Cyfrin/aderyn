use crate::ast::*;
use crate::visitor::ast_visitor::*;
use eyre::Result;

impl Node for EnumValue {
    fn accept(&self, visitor: &mut impl ASTConstVisitor) -> Result<()> {
        visitor.visit_enum_value(self)?;
        visitor.end_visit_enum_value(self)
    }
    fn accept_id(&self, visitor: &mut impl ASTConstVisitor) -> Result<()> {
        visitor.visit_node_id(Some(self.id))?;
        Ok(())
    }
}

impl Node for EnumDefinition {
    fn accept(&self, visitor: &mut impl ASTConstVisitor) -> Result<()> {
        if visitor.visit_enum_definition(self)? {
            list_accept(&self.members, visitor)?;
        }
        self.accept_metadata(visitor)?;
        visitor.end_visit_enum_definition(self)
    }
    fn accept_metadata(&self, visitor: &mut impl ASTConstVisitor) -> Result<()> {
        let member_ids = &self
            .members
            .iter()
            .map(|x| x.id)
            .collect::<Vec<_>>()
            .clone();
        visitor.visit_immediate_children(self.id, member_ids.clone())?;
        Ok(())
    }
    fn accept_id(&self, visitor: &mut impl ASTConstVisitor) -> Result<()> {
        visitor.visit_node_id(Some(self.id))?;
        Ok(())
    }
}
