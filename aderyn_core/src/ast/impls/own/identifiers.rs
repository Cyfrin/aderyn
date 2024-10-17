use crate::ast::*;
use std::hash::{Hash, Hasher};

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
