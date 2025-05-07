use crate::{ast::*, context::browser::is_extcallish};

impl FunctionDefinition {
    /// The kind of function this node defines.
    #[inline]
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
    #[inline]
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

    /// HACK
    /// Internal functions don't have function selectors, because it can have parameters like
    /// storage pointers. In order to identify internal functions that override other internal
    /// functions we must be able to pick a combination of type strings and func names to do the
    /// same.
    ///
    /// TODO: Find a better way
    pub fn selectorish(&self) -> String {
        let func_name = self.name.to_string();
        let mut t = String::new();
        for param in self.parameters.parameters.iter() {
            if let Some(ts) = param.type_descriptions.type_string.as_ref() {
                t.push_str(ts);
            }
            t.push('!');
            if let Some(ti) = param.type_descriptions.type_identifier.as_ref() {
                t.push_str(ti);
            }
            t.push('@');
        }
        func_name + ":" + &t
    }
}

impl ModifierDefinition {
    /// HACK
    /// Internal functions don't have function selectors, because it can have parameters like
    /// storage pointers. In order to identify internal functions that override other internal
    /// functions we must be able to pick a combination of type strings and func names to do the
    /// same.
    ///
    /// TODO: Find a better way
    pub fn selectorish(&self) -> String {
        let func_name = self.name.to_string();
        let mut t = String::new();
        for param in self.parameters.parameters.iter() {
            if let Some(ts) = param.type_descriptions.type_string.as_ref() {
                t.push_str(ts);
            }
            t.push('!');
            if let Some(ti) = param.type_descriptions.type_identifier.as_ref() {
                t.push_str(ti);
            }
            t.push('@');
        }
        func_name + ":" + &t
    }
}

impl FunctionCall {
    /// DO NOT USE
    /// It doesn't work as expected. This was more so crafted for one specific detector and code
    /// needs to be migrated.
    pub fn is_extcallish(&self) -> bool {
        is_extcallish(self.into())
    }

    /// Internal call made to -
    /// * Internal Library function
    /// * Public/Private/Internal contract function
    ///
    ///  Also see [`FunctionCall::suspected_target_function`]
    #[inline]
    pub fn is_internal_call(&self) -> Option<bool> {
        if self.kind != FunctionCallKind::FunctionCall {
            return Some(false);
        }
        // The most common forms of expressions when making a function call is
        // 1) xyz()
        // 2) A.xyz() where A is super or any parent class or library name or a something on which
        //    library is being used for. (using lib for uint8) .... 6.xyz()
        match self.expression.as_ref() {
            Expression::Identifier(Identifier {
                type_descriptions: TypeDescriptions { type_identifier: Some(ty_ident), .. },
                ..
            })
            | Expression::MemberAccess(MemberAccess {
                type_descriptions: TypeDescriptions { type_identifier: Some(ty_ident), .. },
                ..
            }) => Some(ty_ident.starts_with("t_function_internal")),
            _ => None, // TODO: Exhaust these enums
        }
    }
}

impl FunctionCallOptions {
    pub fn is_extcallish(&self) -> bool {
        is_extcallish(self.into())
    }
}

impl VariableDeclaration {
    /// Returns the mutability of the variable that was declared.
    ///
    /// This is a helper to check variable mutability across Solidity versions.
    #[inline]
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
    #[inline]
    pub fn function_definitions(&self) -> Vec<&FunctionDefinition> {
        self.nodes
            .iter()
            .filter_map(|node| {
                if let ContractDefinitionNode::FunctionDefinition(function_definition) = node {
                    Some(function_definition)
                } else {
                    None
                }
            })
            .collect()
    }

    #[inline]
    pub fn modifier_definitions(&self) -> Vec<&ModifierDefinition> {
        self.nodes
            .iter()
            .filter_map(|node| {
                if let ContractDefinitionNode::ModifierDefinition(modifier_definition) = node {
                    Some(modifier_definition)
                } else {
                    None
                }
            })
            .collect()
    }

    pub fn top_level_variables(&self) -> Vec<&VariableDeclaration> {
        self.nodes
            .iter()
            .filter_map(|node| {
                if let ContractDefinitionNode::VariableDeclaration(modifier_definition) = node {
                    Some(modifier_definition)
                } else {
                    None
                }
            })
            .collect()
    }

    #[inline(always)]
    pub fn is_deployable_contract(&self) -> bool {
        self.kind == ContractKind::Contract && !self.is_abstract
    }
}

impl IdentifierOrIdentifierPath {
    #[inline]
    pub fn name(&self) -> String {
        match self {
            IdentifierOrIdentifierPath::Identifier(identifier) => identifier.name.clone(),
            IdentifierOrIdentifierPath::IdentifierPath(identifier_path) => {
                identifier_path.name.clone()
            }
        }
    }

    #[inline]
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
    #[inline]
    pub fn name(&self) -> Option<String> {
        match self {
            UserDefinedTypeNameOrIdentifierPath::UserDefinedTypeName(node) => node.name.clone(),
            UserDefinedTypeNameOrIdentifierPath::IdentifierPath(node) => Some(node.name.clone()),
        }
    }
}

impl Expression {
    #[inline]
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
