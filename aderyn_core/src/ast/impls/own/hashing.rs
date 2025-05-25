use crate::ast::*;
use std::hash::{Hash, Hasher};

// Identifier
impl PartialEq for Identifier {
    fn eq(&self, other: &Self) -> bool {
        self.argument_types.eq(&other.argument_types)
            && self.name.eq(&other.name)
            && self.overloaded_declarations.eq(&other.overloaded_declarations)
            && self.referenced_declaration.eq(&other.referenced_declaration)
            && self.type_descriptions.eq(&other.type_descriptions)
    }
}

impl Hash for Identifier {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.argument_types.hash(state);
        self.name.hash(state);
        self.overloaded_declarations.hash(state);
        self.referenced_declaration.hash(state);
        self.type_descriptions.hash(state);
        self.src.hash(state);
        self.id.hash(state);
    }
}

// Identifier Path
impl PartialEq for IdentifierPath {
    fn eq(&self, other: &Self) -> bool {
        self.name.eq(&other.name) && self.referenced_declaration.eq(&other.referenced_declaration)
    }
}

impl Hash for IdentifierPath {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.name.hash(state);
        self.referenced_declaration.hash(state);
        self.src.hash(state);
        self.id.hash(state);
    }
}

// ElementaryTypeName
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

// UserDefinedTypeName
impl PartialEq for UserDefinedTypeName {
    fn eq(&self, other: &Self) -> bool {
        self.referenced_declaration.eq(&other.referenced_declaration)
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
