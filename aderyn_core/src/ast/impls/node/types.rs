use crate::ast::*;
use crate::visitor::ast_visitor::*;
use eyre::Result;
use std::fmt::Display;
use std::hash::{Hash, Hasher};

impl Node for TypeName {
    fn accept(&self, visitor: &mut impl ASTConstVisitor) -> Result<()> {
        match self {
            TypeName::FunctionTypeName(function_type_name) => function_type_name.accept(visitor),
            TypeName::ArrayTypeName(array_type_name) => array_type_name.accept(visitor),
            TypeName::Mapping(mapping) => mapping.accept(visitor),
            TypeName::UserDefinedTypeName(user_defined_type_name) => {
                user_defined_type_name.accept(visitor)
            }
            TypeName::ElementaryTypeName(elementary_type_name) => {
                elementary_type_name.accept(visitor)
            }
            TypeName::Raw(_) => Ok(()),
        }
    }
}

impl Display for TypeName {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            TypeName::ElementaryTypeName(elementary_type_name) => elementary_type_name.fmt(f),
            TypeName::UserDefinedTypeName(user_defined_type_name) => user_defined_type_name.fmt(f),
            TypeName::ArrayTypeName(array_type_name) => array_type_name.fmt(f),
            TypeName::Mapping(mapping) => mapping.fmt(f),
            TypeName::Raw(string) => string.fmt(f),
            _ => unimplemented!(),
        }
    }
}

impl Node for ElementaryTypeName {
    fn accept(&self, visitor: &mut impl ASTConstVisitor) -> Result<()> {
        visitor.visit_elementary_type_name(self)?;
        visitor.end_visit_elementary_type_name(self)
    }
}

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

impl Display for ElementaryTypeName {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(self.name.as_str())?;

        if let Some(state_mutability) = self.state_mutability {
            if state_mutability != StateMutability::NonPayable {
                f.write_fmt(format_args!(" {state_mutability}"))?;
            }
        }

        Ok(())
    }
}

impl Node for UserDefinedTypeName {
    fn accept(&self, visitor: &mut impl ASTConstVisitor) -> Result<()> {
        if visitor.visit_user_defined_type_name(self)? && self.path_node.is_some() {
            self.path_node.as_ref().unwrap().accept(visitor)?;
        }
        visitor.end_visit_user_defined_type_name(self)
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

impl Display for UserDefinedTypeName {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        if let Some(path_node) = self.path_node.as_ref() {
            f.write_fmt(format_args!("{path_node}"))
        } else {
            f.write_fmt(format_args!("{}", self.name.as_deref().unwrap_or("")))
        }
    }
}

impl Node for FunctionTypeName {
    fn accept(&self, visitor: &mut impl ASTConstVisitor) -> Result<()> {
        if visitor.visit_function_type_name(self)? {
            self.parameter_types.accept(visitor)?;
            self.return_parameter_types.accept(visitor)?;
        }
        visitor.end_visit_function_type_name(self)
    }
}

impl Node for ArrayTypeName {
    fn accept(&self, visitor: &mut impl ASTConstVisitor) -> Result<()> {
        if visitor.visit_array_type_name(self)? {
            self.base_type.accept(visitor)?;
            if let Some(length) = self.length.as_ref() {
                length.accept(visitor)?;
            }
        }
        visitor.end_visit_array_type_name(self)
    }
}

impl Display for ArrayTypeName {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_fmt(format_args!("{}", self.base_type))?;
        f.write_str("[")?;

        if let Some(length) = self.length.as_ref() {
            f.write_fmt(format_args!("{length}"))?;
        }

        f.write_str("]")
    }
}

impl Node for Mapping {
    fn accept(&self, visitor: &mut impl ASTConstVisitor) -> Result<()> {
        if visitor.visit_mapping(self)? {
            self.key_type.accept(visitor)?;
            self.value_type.accept(visitor)?;
        }
        visitor.end_visit_mapping(self)
    }
}

impl Display for Mapping {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_fmt(format_args!(
            "mapping({} => {})",
            self.key_type, self.value_type
        ))
    }
}
