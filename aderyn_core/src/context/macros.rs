macro_rules! generate_capturable_methods {
    ($( $name:ident ),* $(,)*) => {

        #[derive(Clone)]
        pub enum Capturable {
            $($name($name),)*
            YulFunctionCall(YulFunctionCall),
            YulIdentifier(YulIdentifier),
            YulLiteral(YulLiteral),
            ASTNode(ASTNode),
        }

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

        impl From<ASTNode> for Capturable {
            fn from(value: ASTNode) -> Self {
                Self::ASTNode(value)
            }
        }

        impl From<&ASTNode> for Capturable {
            fn from(value: &ASTNode) -> Self {
                Self::ASTNode(value.clone())
            }
        }


        impl Capturable {
            pub fn make_key(&self, context: &WorkspaceContext) -> (String, usize, String) {
                match self {
                    Self::ASTNode(node) => context.get_node_sort_key(node),
                    Self::YulFunctionCall(n) => context.get_node_sort_key(&n.into()),
                    Self::YulIdentifier(n) => context.get_node_sort_key(&n.into()),
                    Self::YulLiteral(n) => context.get_node_sort_key(&n.into()),
                    $(Self::$name(n) => context.get_node_sort_key(&n.into()),)*
                }
            }
            pub fn id(&self) -> Option<NodeID> {
                match self {
                    Self::ASTNode(ast_node) => ast_node.id(),
                    Self::YulFunctionCall(_) => None,
                    Self::YulIdentifier(_) => None,
                    Self::YulLiteral(_) => None,
                    $(Self::$name(n) => Some(n.id),)*
                }
            }
        }


    };
}

macro_rules! generate_get_source_unit {

    ($( $name:ident => $storage_var:ident ),* $(,)*) => {

        impl WorkspaceContext {
            pub fn get_source_unit_from_child_node(&self, node: &ASTNode) -> Option<&SourceUnit> {
                let source_unit_id = match node {
                    ASTNode::SourceUnit(node) => Some(node.id),
                    $(
                        ASTNode::$name(node) => self
                            .$storage_var
                            .get(node)
                            .map(|context| context.source_unit_id),
                    )*
                };
                // iterate through self.source_units until the source unit with the id matching `source_unit_id` is found, then return its `absolute_path`
                source_unit_id.and_then(|id| {
                    self.source_units_context
                        .iter()
                        .find(|source_unit| source_unit.id == id)
                })
            }
        }

    };

}

pub(crate) use generate_capturable_methods;
pub(crate) use generate_get_source_unit;
