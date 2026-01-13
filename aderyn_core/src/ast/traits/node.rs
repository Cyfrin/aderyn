use crate::ast::*;
use eyre::Result;

pub trait Node {
    /// [`Node::accept`] is designed to propagate
    fn accept(&self, _visitor: &mut impl ASTConstVisitor) -> Result<()> {
        Ok(())
    }
    /// [`Node::accept_metadata`] is designed to propagate into the AST subtree
    /// although it doesn't happen by itself. [`Node::accept`] triggers the propagation
    fn accept_metadata(&self, _visitor: &mut impl ASTConstVisitor) -> Result<()> {
        Ok(())
    }
    /// [`Node::accept_id`] is not designed to propagate into the AST subtree
    fn accept_id(&self, _visitor: &mut impl ASTConstVisitor) -> Result<()> {
        Ok(())
    }
}

pub fn list_accept(list: &Vec<impl Node>, visitor: &mut impl ASTConstVisitor) -> Result<()> {
    for elem in list {
        elem.accept(visitor)?;
    }
    Ok(())
}
