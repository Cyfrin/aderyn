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
            CfgNodeDescriptor::Start => CfgNodeKind::Void,
            CfgNodeDescriptor::End => CfgNodeKind::Void,

            // Primitives
            CfgNodeDescriptor::VariableDeclarationStatement => CfgNodeKind::Primitive,
            CfgNodeDescriptor::ExpressionStatement => CfgNodeKind::Primitive,

            // Reducibles
            CfgNodeDescriptor::Block(_) => CfgNodeKind::Reducible,
        }
    }
}
