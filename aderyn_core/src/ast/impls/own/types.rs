use crate::ast::*;
use std::hash::{Hash, Hasher};

impl PartialEq for ElementaryTypeName {
    fn eq(&self, other: &Self) -> bool {
        self.state_mutability.eq(&other.state_mutability)
            && self.type_descriptions.eq(&other.type_descriptions)
    }
}

impl Hash for ElementaryTypeName {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.state_mutability.hash(state);
        self.name.hash(state);
        self.type_descriptions.hash(state);
    }
}

impl PartialEq for UserDefinedTypeName {
    fn eq(&self, other: &Self) -> bool {
        self.referenced_declaration
            .eq(&other.referenced_declaration)
    }
}

impl Hash for UserDefinedTypeName {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.path_node.hash(state);
        self.referenced_declaration.hash(state);
        self.name.hash(state);
        self.type_descriptions.hash(state);
    }
}

impl TypeName {
    pub fn get_node_id(&self) -> Option<NodeID> {
        match self {
            TypeName::FunctionTypeName(node) => Some(node.id),
            TypeName::ArrayTypeName(node) => Some(node.id),
            TypeName::Mapping(node) => Some(node.id),
            TypeName::UserDefinedTypeName(node) => Some(node.id),
            TypeName::ElementaryTypeName(node) => Some(node.id),
            TypeName::Raw(_) => None,
        }
    }
}
