use crate::ast::*;

impl FunctionDefinition {
    /// The kind of function this node defines.
    pub fn kind(&self) -> &FunctionKind {
        if let Some(kind) = &self.kind {
            kind
        } else if self.is_constructor {
            &FunctionKind::Constructor
        } else {
            &FunctionKind::Function
        }
    }

    /// The state mutability of the function.
    ///
    /// Note: Before Solidity 0.5.x, this is an approximation, as there was no distinction between
    /// `view` and `pure`.
    pub fn state_mutability(&self) -> &StateMutability {
        if let Some(state_mutability) = &self.state_mutability {
            state_mutability
        } else if self.is_declared_const {
            &StateMutability::View
        } else if self.is_payable {
            &StateMutability::Payable
        } else {
            &StateMutability::NonPayable
        }
    }
}

impl VariableDeclaration {
    /// Returns the mutability of the variable that was declared.
    ///
    /// This is a helper to check variable mutability across Solidity versions.
    pub fn mutability(&self) -> Option<&Mutability> {
        if let Some(mutability) = &self.mutability {
            Some(mutability)
        } else if self.constant {
            Some(&Mutability::Constant)
        } else if self.state_variable {
            Some(&Mutability::Mutable)
        } else {
            None
        }
    }
}

impl ContractDefinition {
    pub fn function_definitions(&self) -> Vec<&FunctionDefinition> {
        let mut result = vec![];
        for node in self.nodes.iter() {
            if let ContractDefinitionNode::FunctionDefinition(function_definition) = node {
                result.push(function_definition);
            }
        }
        result
    }
}

impl IdentifierOrIdentifierPath {
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
                Some(identifier_path.referenced_declaration)
            }
        }
    }
}

impl UserDefinedTypeNameOrIdentifierPath {
    pub fn name(&self) -> Option<String> {
        match self {
            UserDefinedTypeNameOrIdentifierPath::UserDefinedTypeName(node) => node.name.clone(),
            UserDefinedTypeNameOrIdentifierPath::IdentifierPath(node) => Some(node.name.clone()),
        }
    }
}

impl Expression {
    pub fn type_descriptions(&self) -> Option<&TypeDescriptions> {
        match self {
            Expression::Literal(Literal { type_descriptions, .. }) => Some(type_descriptions),
            Expression::Identifier(Identifier { type_descriptions, .. }) => Some(type_descriptions),
            Expression::UnaryOperation(UnaryOperation { type_descriptions, .. }) => {
                Some(type_descriptions)
            }
            Expression::BinaryOperation(BinaryOperation { type_descriptions, .. }) => {
                Some(type_descriptions)
            }
            Expression::Conditional(Conditional { type_descriptions, .. }) => {
                Some(type_descriptions)
            }
            Expression::Assignment(Assignment { type_descriptions, .. }) => Some(type_descriptions),
            Expression::FunctionCall(FunctionCall { type_descriptions, .. }) => {
                Some(type_descriptions)
            }
            Expression::FunctionCallOptions(FunctionCallOptions { type_descriptions, .. }) => {
                Some(type_descriptions)
            }
            Expression::IndexAccess(IndexAccess { type_descriptions, .. }) => {
                Some(type_descriptions)
            }
            Expression::IndexRangeAccess(IndexRangeAccess { type_descriptions, .. }) => {
                Some(type_descriptions)
            }
            Expression::MemberAccess(MemberAccess { type_descriptions, .. }) => {
                Some(type_descriptions)
            }
            Expression::ElementaryTypeNameExpression(ElementaryTypeNameExpression {
                type_descriptions,
                ..
            }) => Some(type_descriptions),
            Expression::TupleExpression(TupleExpression { type_descriptions, .. }) => {
                Some(type_descriptions)
            }
            Expression::NewExpression(NewExpression { type_descriptions, .. }) => {
                Some(type_descriptions)
            }
        }
    }
}
