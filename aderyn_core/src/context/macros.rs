macro_rules! generate_capturable_methods {
    ($( $name:ident ),* $(,)*) => {

        #[derive(Clone)]
        pub enum Capturable {
            $($name($name),)*
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
