use crate::ast::*;

impl IdentifierOrIdentifierPath {
    pub fn get_node_id(&self) -> NodeID {
        match self {
            IdentifierOrIdentifierPath::Identifier(n) => n.id,
            IdentifierOrIdentifierPath::IdentifierPath(n) => n.id,
        }
    }

    pub fn name(&self) -> String {
        match self {
            IdentifierOrIdentifierPath::Identifier(identifier) => identifier.name.clone(),
            IdentifierOrIdentifierPath::IdentifierPath(identifier_path) => {
                identifier_path.name.clone()
            }
        }
    }

    pub fn referenced_declaration(&self) -> Option<NodeID> {
        match self {
            IdentifierOrIdentifierPath::Identifier(identifier) => identifier.referenced_declaration,
            IdentifierOrIdentifierPath::IdentifierPath(identifier_path) => {
                identifier_path.referenced_declaration
            }
        }
    }
}
