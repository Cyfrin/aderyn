use crate::ast::*;

impl UserDefinedTypeNameOrIdentifierPath {
    pub fn name(&self) -> Option<String> {
        match self {
            UserDefinedTypeNameOrIdentifierPath::UserDefinedTypeName(node) => node.name.clone(),
            UserDefinedTypeNameOrIdentifierPath::IdentifierPath(node) => Some(node.name.clone()),
        }
    }

    pub fn get_node_id(&self) -> Option<NodeID> {
        match self {
            UserDefinedTypeNameOrIdentifierPath::UserDefinedTypeName(node) => Some(node.id),
            UserDefinedTypeNameOrIdentifierPath::IdentifierPath(node) => Some(node.id),
        }
    }
}
