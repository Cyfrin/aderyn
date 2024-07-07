macro_rules! generate_capture_methods {
    ($( $name:ident ),* $(,)*) => {
        $(
            impl From<$name> for Capturable {
                fn from(value: $name) -> Self {
                    Self::$name(value)
                }
            }

            impl From<&$name> for Capturable {
                fn from(value: &$name) -> Self {
                    Self::$name(value.clone())
                }
            }
        )*
    };
}

macro_rules! generate_ast_methods {
    ($( $name:ident ),* $(,)*) => {
        $(
            impl From<$name> for ASTNode {
                fn from(value: $name) -> Self {
                    ASTNode::$name(value)
                }
            }

            impl From<&$name> for ASTNode {
                fn from(value: &$name) -> Self {
                    ASTNode::$name(value.clone())
                }
            }
        )*

        impl ASTNode {
            pub fn node_type(&self) -> NodeType {
                match self {
                    $(ASTNode::$name(_) => NodeType::$name,)*
                }
            }
            pub fn id(&self) -> Option<NodeID> {
                match self {
                    $(ASTNode::$name(n) => Some(n.id),)*
                }
            }
        }

        impl Node for ASTNode {
            fn accept(&self, visitor: &mut impl ASTConstVisitor) -> eyre::Result<()> {
                match self {
                    $(ASTNode::$name(n) => n.accept(visitor),)*
                }
            }
            fn accept_metadata(&self, visitor: &mut impl ASTConstVisitor) -> eyre::Result<()> {
                match self {
                    $(ASTNode::$name(n) => n.accept_metadata(visitor),)*
                }
            }
            fn accept_id(&self, visitor: &mut impl ASTConstVisitor) -> Result<()> {
                visitor.visit_node_id(self.id())?;
                Ok(())
            }
        }

    };
}

pub(crate) use generate_ast_methods;
pub(crate) use generate_capture_methods;
