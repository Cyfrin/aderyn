macro_rules! generate_extraction_library {
    ($( $name:ident | $visit_method:ident => $node:ident |),* $(,)*) => {
        $(
            #[derive(Default)]
            pub struct $name {
                pub extracted: Vec<$node>,
            }

            impl $name {
                pub fn from<T: Node + ?Sized>(node: &T) -> Self {
                    let mut extractor: $name = Self::default();
                    node.accept(&mut extractor).unwrap_or_default();
                    extractor
                }
            }

            impl ASTConstVisitor for $name {
                fn $visit_method(&mut self, node: &$node) -> Result<bool> {
                    self.extracted.push(node.clone());
                    Ok(true)
                }
            }
        )*
    };
}

pub(crate) use generate_extraction_library;
