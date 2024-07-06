use crate::ast::*;
use crate::visitor::ast_visitor::*;
use eyre::Result;


impl UserDefinedTypeNameOrIdentifierPath {
    pub fn name(&self) -> Option<String> {
        match self {
            UserDefinedTypeNameOrIdentifierPath::UserDefinedTypeName(node) => node.name.clone(),
            UserDefinedTypeNameOrIdentifierPath::IdentifierPath(node) => Some(node.name.clone()),
        }
    }

    pub fn get_node_id(&self) -> Option<NodeID> {
        match self {
            UserDefinedTypeNameOrIdentifierPath::UserDefinedTypeName(_) => None,
            UserDefinedTypeNameOrIdentifierPath::IdentifierPath(node) => Some(node.id),
        }
    }
}

impl Node for UsingForDirective {
    fn accept(&self, visitor: &mut impl ASTConstVisitor) -> Result<()> {
        if visitor.visit_using_for_directive(self)? {
            // TODO there is a deviation. Missing FuntionsOrLibrary
            if self.library_name.is_some() {
                match self.library_name.as_ref().unwrap() {
                    UserDefinedTypeNameOrIdentifierPath::UserDefinedTypeName(_) => {}
                    UserDefinedTypeNameOrIdentifierPath::IdentifierPath(identifier_path) => {
                        identifier_path.accept(visitor)?;
                    }
                };
            }
            if self.type_name.is_some() {
                self.type_name.as_ref().unwrap().accept(visitor)?;
            }
            self.accept_metadata(visitor)?;
        }
        visitor.end_visit_using_for_directive(self)
    }
    fn accept_metadata(&self, visitor: &mut impl ASTConstVisitor) -> Result<()> {
        if self.library_name.is_some() {
            match self.library_name.as_ref().unwrap() {
                UserDefinedTypeNameOrIdentifierPath::UserDefinedTypeName(_) => {}
                UserDefinedTypeNameOrIdentifierPath::IdentifierPath(identifier_path) => {
                    visitor.visit_immediate_children(self.id, vec![identifier_path.id])?;
                }
            };
        }
        Ok(())
    }
    fn accept_id(&self, visitor: &mut impl ASTConstVisitor) -> Result<()> {
        visitor.visit_node_id(Some(self.id))?;
        Ok(())
    }
}
