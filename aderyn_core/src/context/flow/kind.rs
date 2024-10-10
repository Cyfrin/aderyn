//! Following are the types of statements that are to be considered when building a
//! Control Flow graph
//!
//! REDUCIBLES
//!
//! Step-in
//! -------
//! Block
//! UncheckedBlock
//!
//! Flow
//! ----
//! DoWhileStatement
//! IfStatement
//! ForStatement
//! WhileStatement
//!
//! ----------------------------
//!
//! PRIMITIVES
//!
//! Substitute
//! ----------
//! PlaceholderStatement
//!
//! Jumper
//! ------
//! Break
//! Continue
//! Return
//!
//! Regular
//! ------
//! EmitStatement
//! RevertStatement
//! ExpressionStatement
//! InlineAssembly
//! VariableDeclarationStatement
//! TryStatement

use super::CfgNodeDescriptor;

#[derive(PartialEq, Clone, Copy)]
pub enum CfgNodeKind {
    Void,
    Reducible,
    Primitive,
}

impl CfgNodeDescriptor {
    pub fn kind(&self) -> CfgNodeKind {
        match self {
            // Void nodes
            CfgNodeDescriptor::Start(_) => CfgNodeKind::Void,
            CfgNodeDescriptor::End(_) => CfgNodeKind::Void,

            // Primitives
            CfgNodeDescriptor::VariableDeclarationStatement(_) => CfgNodeKind::Primitive,
            CfgNodeDescriptor::ExpressionStatement(_) => CfgNodeKind::Primitive,
            CfgNodeDescriptor::PlaceholderStatement(_) => CfgNodeKind::Primitive,
            CfgNodeDescriptor::Break(_) => CfgNodeKind::Primitive,
            CfgNodeDescriptor::Continue(_) => CfgNodeKind::Primitive,
            CfgNodeDescriptor::Return(_) => CfgNodeKind::Primitive,
            CfgNodeDescriptor::EmitStatement(_) => CfgNodeKind::Primitive,
            CfgNodeDescriptor::RevertStatement(_) => CfgNodeKind::Primitive,
            CfgNodeDescriptor::InlineAssembly(_) => CfgNodeKind::Primitive,
            CfgNodeDescriptor::TryStatement(_) => CfgNodeKind::Primitive,
            CfgNodeDescriptor::IfStatementCondition(_) => CfgNodeKind::Primitive,
            CfgNodeDescriptor::WhileStatementCondition(_) => CfgNodeKind::Primitive,
            CfgNodeDescriptor::ForStatementCondition(_) => CfgNodeKind::Primitive,
            CfgNodeDescriptor::DoWhileStatementCondition(_) => CfgNodeKind::Primitive,

            // Reducibles
            CfgNodeDescriptor::Block(_) => CfgNodeKind::Reducible,
            CfgNodeDescriptor::UncheckedBlock(_) => CfgNodeKind::Reducible,
            CfgNodeDescriptor::IfStatement(_) => CfgNodeKind::Reducible,
            CfgNodeDescriptor::WhileStatement(_) => CfgNodeKind::Reducible,
            CfgNodeDescriptor::ForStatement(_) => CfgNodeKind::Reducible,
            CfgNodeDescriptor::DoWhileStatement(_) => CfgNodeKind::Reducible,
        }
    }
}
