use super::*;
use crate::visitor::ast_visitor::*;
use eyre::Result;
use serde::{Deserialize, Serialize};
use std::fmt::Display;

#[derive(Clone, Debug, Deserialize, Serialize, PartialEq, PartialOrd, Eq, Ord, Hash)]
#[serde(rename_all = "lowercase")]
pub enum ContractKind {
    Contract,
    Interface,
    Library,
}

impl Display for ContractKind {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_fmt(format_args!("{}", format!("{self:?}").to_lowercase()))
    }
}

#[derive(Clone, Debug, Eq, Serialize, Deserialize, PartialEq, Hash)]
#[serde(tag = "nodeType")]
pub enum ContractDefinitionNode {
    UsingForDirective(UsingForDirective),
    StructDefinition(StructDefinition),
    EnumDefinition(EnumDefinition),
    VariableDeclaration(VariableDeclaration),
    EventDefinition(EventDefinition),
    FunctionDefinition(FunctionDefinition),
    ModifierDefinition(ModifierDefinition),
    ErrorDefinition(ErrorDefinition),
    UserDefinedValueTypeDefinition(UserDefinedValueTypeDefinition),
}

impl Node for ContractDefinitionNode {
    fn accept(&self, visitor: &mut impl ASTConstVisitor) -> Result<()> {
        match self {
            ContractDefinitionNode::UsingForDirective(using_for_directive) => {
                using_for_directive.accept(visitor)
            }
            ContractDefinitionNode::StructDefinition(struct_definition) => {
                struct_definition.accept(visitor)
            }
            ContractDefinitionNode::EnumDefinition(enum_definition) => {
                enum_definition.accept(visitor)
            }
            ContractDefinitionNode::VariableDeclaration(variable_declaration) => {
                variable_declaration.accept(visitor)
            }
            ContractDefinitionNode::EventDefinition(event_definition) => {
                event_definition.accept(visitor)
            }
            ContractDefinitionNode::FunctionDefinition(function_definition) => {
                function_definition.accept(visitor)
            }
            ContractDefinitionNode::ModifierDefinition(modifier_definition) => {
                modifier_definition.accept(visitor)
            }
            ContractDefinitionNode::ErrorDefinition(error_definition) => {
                error_definition.accept(visitor)
            }
            ContractDefinitionNode::UserDefinedValueTypeDefinition(
                user_defined_value_type_definition,
            ) => user_defined_value_type_definition.accept(visitor),
        }
    }

    fn accept_id(&self, visitor: &mut impl ASTConstVisitor) -> Result<()> {
        visitor.visit_node_id(self.get_node_id())?;
        Ok(())
    }
}

impl ContractDefinitionNode {
    pub fn get_node_id(&self) -> Option<NodeID> {
        match self {
            ContractDefinitionNode::UsingForDirective(using_for_directive) => {
                Some(using_for_directive.id)
            }
            ContractDefinitionNode::StructDefinition(struct_definition) => {
                Some(struct_definition.id)
            }
            ContractDefinitionNode::EnumDefinition(enum_definition) => Some(enum_definition.id),
            ContractDefinitionNode::VariableDeclaration(variable_declaration) => {
                Some(variable_declaration.id)
            }
            ContractDefinitionNode::EventDefinition(event_definition) => Some(event_definition.id),
            ContractDefinitionNode::FunctionDefinition(function_definition) => {
                Some(function_definition.id)
            }
            ContractDefinitionNode::ModifierDefinition(modifier_definition) => {
                Some(modifier_definition.id)
            }
            ContractDefinitionNode::ErrorDefinition(error_definition) => Some(error_definition.id),
            ContractDefinitionNode::UserDefinedValueTypeDefinition(
                user_defined_value_type_definition,
            ) => Some(user_defined_value_type_definition.id),
        }
    }
}

impl Display for ContractDefinitionNode {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ContractDefinitionNode::UsingForDirective(using_for_directive) => {
                using_for_directive.fmt(f)
            }
            ContractDefinitionNode::StructDefinition(struct_definition) => struct_definition.fmt(f),
            ContractDefinitionNode::EnumDefinition(enum_definition) => enum_definition.fmt(f),
            ContractDefinitionNode::VariableDeclaration(variable_declaration) => {
                variable_declaration.fmt(f)
            }
            ContractDefinitionNode::EventDefinition(event_definition) => event_definition.fmt(f),
            ContractDefinitionNode::FunctionDefinition(function_definition) => {
                function_definition.fmt(f)
            }
            ContractDefinitionNode::ModifierDefinition(modifier_definition) => {
                modifier_definition.fmt(f)
            }
            ContractDefinitionNode::ErrorDefinition(error_definition) => error_definition.fmt(f),
            ContractDefinitionNode::UserDefinedValueTypeDefinition(
                user_defined_value_type_definition,
            ) => user_defined_value_type_definition.fmt(f),
        }
    }
}

#[derive(Clone, Debug, Deserialize, Eq, Serialize, PartialEq, Hash)]
#[serde(rename_all = "camelCase")]
pub struct InheritanceSpecifier {
    pub base_name: IdentifierPath,
    pub arguments: Option<Vec<Expression>>,
    pub src: String,
    pub id: NodeID,
}

impl Node for InheritanceSpecifier {
    fn accept(&self, visitor: &mut impl ASTConstVisitor) -> Result<()> {
        if visitor.visit_inheritance_specifier(self)? {
            self.base_name.accept(visitor)?;
            if self.arguments.is_some() {
                list_accept(self.arguments.as_ref().unwrap(), visitor)?;
            }
        }
        self.accept_metadata(visitor)?;
        visitor.end_visit_inheritance_specifier(self)
    }
    fn accept_metadata(&self, visitor: &mut impl ASTConstVisitor) -> Result<()> {
        visitor.visit_immediate_children(self.id, vec![self.base_name.id])?;
        let mut argument_ids: Vec<NodeID> = vec![];
        if let Some(arguments) = &self.arguments {
            for expression in arguments {
                let node_id = expression.get_node_id();
                if let Some(node_id) = node_id {
                    argument_ids.push(node_id)
                }
            }
        }
        visitor.visit_immediate_children(self.id, argument_ids)?;
        Ok(())
    }
    fn accept_id(&self, visitor: &mut impl ASTConstVisitor) -> Result<()> {
        visitor.visit_node_id(Some(self.id))?;
        Ok(())
    }
}

impl Display for InheritanceSpecifier {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_fmt(format_args!("{}", self.base_name))?;

        if let Some(arguments) = self.arguments.as_ref() {
            f.write_str("(")?;

            for (i, argument) in arguments.iter().enumerate() {
                f.write_fmt(format_args!(
                    "{}{}",
                    match i {
                        0 => "",
                        _ => ", ",
                    },
                    argument,
                ))?;
            }

            f.write_str(")")?;
        }

        Ok(())
    }
}

#[derive(Clone, Debug, Deserialize, Eq, Serialize, PartialEq, Hash)]
#[serde(rename_all = "camelCase")]
pub struct ContractDefinition {
    pub name: String,
    pub name_location: Option<String>,
    pub documentation: Option<Documentation>,
    #[serde(rename = "contractKind")]
    pub kind: ContractKind,
    #[serde(rename = "abstract")]
    pub is_abstract: Option<bool>,
    pub base_contracts: Vec<InheritanceSpecifier>,
    pub contract_dependencies: Vec<NodeID>,
    pub used_errors: Option<Vec<NodeID>>,
    pub nodes: Vec<ContractDefinitionNode>,
    pub scope: NodeID,
    pub fully_implemented: Option<bool>,
    pub linearized_base_contracts: Option<Vec<NodeID>>,
    pub src: String,
    pub id: NodeID,
}

impl Node for ContractDefinition {
    fn accept(&self, visitor: &mut impl ASTConstVisitor) -> Result<()> {
        if visitor.visit_contract_definition(self)? {
            if self.documentation.is_some() {
                self.documentation.as_ref().unwrap().accept(visitor)?;
            }
            list_accept(&self.base_contracts, visitor)?;
            list_accept(&self.nodes, visitor)?;
        }
        self.accept_metadata(visitor)?;
        visitor.end_visit_contract_definition(self)
    }

    fn accept_metadata(&self, visitor: &mut impl ASTConstVisitor) -> Result<()> {
        // TODO: Skipping documentation for now
        let mut base_contracts_ids = vec![];
        for base_contract in &self.base_contracts {
            base_contracts_ids.push(base_contract.id);
        }
        visitor.visit_immediate_children(self.id, base_contracts_ids)?;
        let mut node_ids = vec![];
        for node in &self.nodes {
            if let Some(node_id) = node.get_node_id() {
                node_ids.push(node_id);
            }
        }
        visitor.visit_immediate_children(self.id, node_ids)?;
        Ok(())
    }

    fn accept_id(&self, visitor: &mut impl ASTConstVisitor) -> Result<()> {
        visitor.visit_node_id(Some(self.id))?;
        Ok(())
    }
}

impl ContractDefinition {
    pub fn using_for_directive(&self, id: NodeID) -> Option<&UsingForDirective> {
        for node in self.nodes.iter() {
            if let ContractDefinitionNode::UsingForDirective(using_for_directive) = node {
                if using_for_directive.id == id {
                    return Some(using_for_directive);
                }
            }
        }

        None
    }

    pub fn using_for_directives(&self) -> Vec<&UsingForDirective> {
        let mut result = vec![];

        for node in self.nodes.iter() {
            if let ContractDefinitionNode::UsingForDirective(using_for_directive) = node {
                result.push(using_for_directive);
            }
        }

        result
    }

    pub fn struct_definition(&self, id: NodeID) -> Option<&StructDefinition> {
        for node in self.nodes.iter() {
            if let ContractDefinitionNode::StructDefinition(struct_definition) = node {
                if id == struct_definition.id {
                    return Some(struct_definition);
                }
            }
        }

        None
    }

    pub fn struct_definitions(&self) -> Vec<&StructDefinition> {
        let mut result = vec![];

        for node in self.nodes.iter() {
            if let ContractDefinitionNode::StructDefinition(struct_definition) = node {
                result.push(struct_definition);
            }
        }

        result
    }

    pub fn enum_definition(&self, id: NodeID) -> Option<&EnumDefinition> {
        for node in self.nodes.iter() {
            if let ContractDefinitionNode::EnumDefinition(enum_definition) = node {
                if id == enum_definition.id {
                    return Some(enum_definition);
                }
            }
        }

        None
    }

    pub fn enum_definitions(&self) -> Vec<&EnumDefinition> {
        let mut result = vec![];

        for node in self.nodes.iter() {
            if let ContractDefinitionNode::EnumDefinition(enum_definition) = node {
                result.push(enum_definition);
            }
        }

        result
    }

    pub fn event_definition(&self, id: NodeID) -> Option<&EventDefinition> {
        for node in self.nodes.iter() {
            if let ContractDefinitionNode::EventDefinition(event_definition) = node {
                if id == event_definition.id {
                    return Some(event_definition);
                }
            }
        }

        None
    }

    pub fn variable_declaration(&self, id: NodeID) -> Option<&VariableDeclaration> {
        for node in self.nodes.iter() {
            if let ContractDefinitionNode::VariableDeclaration(variable_declaration) = node {
                if id == variable_declaration.id {
                    return Some(variable_declaration);
                }
            }
        }

        None
    }

    pub fn variable_declarations(&self) -> Vec<&VariableDeclaration> {
        let mut result = vec![];

        for node in self.nodes.iter() {
            if let ContractDefinitionNode::VariableDeclaration(variable_declaration) = node {
                result.push(variable_declaration);
            }
        }

        result
    }

    pub fn function_definition(&self, id: NodeID) -> Option<&FunctionDefinition> {
        for node in self.nodes.iter() {
            if let ContractDefinitionNode::FunctionDefinition(function_definition) = node {
                if id == function_definition.id {
                    return Some(function_definition);
                }
            }
        }

        None
    }

    pub fn function_definitions(&self) -> Vec<&FunctionDefinition> {
        let mut result = vec![];

        for node in self.nodes.iter() {
            if let ContractDefinitionNode::FunctionDefinition(function_definition) = node {
                result.push(function_definition);
            }
        }

        result
    }

    pub fn modifier_definition(&self, id: NodeID) -> Option<&ModifierDefinition> {
        for node in self.nodes.iter() {
            if let ContractDefinitionNode::ModifierDefinition(modifier_definition) = node {
                if id == modifier_definition.id {
                    return Some(modifier_definition);
                }
            }
        }

        None
    }

    pub fn modifier_definitions(&self) -> Vec<&ModifierDefinition> {
        let mut result = vec![];

        for node in self.nodes.iter() {
            if let ContractDefinitionNode::ModifierDefinition(modifier_definition) = node {
                result.push(modifier_definition);
            }
        }

        result
    }

    pub fn hierarchy_contains_state_variable(
        &self,
        source_units: &[SourceUnit],
        state_variable_id: NodeID,
    ) -> bool {
        // Loop through all of the contracts in the supplied contract's inheritance hierarchy
        if let Some(contract_ids) = self.linearized_base_contracts.as_ref() {
            for &contract_id in contract_ids.iter() {
                // Loop through all of the schema source_units in the project
                for source_unit in source_units.iter() {
                    // Attempt to retrieve the current contract in the inheritance hierarchy from the current schema source_unit
                    let contract_definition = match source_unit.contract_definition(contract_id) {
                        Some(contract_definition) => contract_definition,
                        None => continue,
                    };

                    // Attempt to retrieve the requested state variable from the current contract in the inheritance hierarchy
                    if contract_definition
                        .variable_declaration(state_variable_id)
                        .is_some()
                    {
                        return true;
                    }
                }
            }
        } else if self.variable_declaration(state_variable_id).is_some() {
            return true;
        }

        false
    }

    pub fn get_assigned_state_variables(
        &self,
        source_units: &[SourceUnit],
        _definition_node: &ContractDefinitionNode,
        expression: &Expression,
    ) -> Vec<NodeID> {
        let mut ids = vec![];

        match expression {
            Expression::Identifier(identifier) => {
                if self.hierarchy_contains_state_variable(
                    source_units,
                    identifier.referenced_declaration,
                ) {
                    ids.push(identifier.referenced_declaration);
                }
            }

            Expression::Assignment(assignment) => {
                ids.extend(self.get_assigned_state_variables(
                    source_units,
                    _definition_node,
                    assignment.left_hand_side.as_ref(),
                ));
            }

            Expression::IndexAccess(index_access) => {
                ids.extend(self.get_assigned_state_variables(
                    source_units,
                    _definition_node,
                    index_access.base_expression.as_ref(),
                ));
            }

            Expression::IndexRangeAccess(index_range_access) => {
                ids.extend(self.get_assigned_state_variables(
                    source_units,
                    _definition_node,
                    index_range_access.base_expression.as_ref(),
                ));
            }

            Expression::MemberAccess(member_access) => {
                ids.extend(self.get_assigned_state_variables(
                    source_units,
                    _definition_node,
                    member_access.expression.as_ref(),
                ));
            }

            Expression::TupleExpression(tuple_expression) => {
                for component in tuple_expression.components.iter().flatten() {
                    ids.extend(self.get_assigned_state_variables(
                        source_units,
                        _definition_node,
                        component,
                    ));
                }
            }

            _ => (),
        }

        ids
    }

    pub fn definition_node_location(
        &self,
        source_line: usize,
        definition_node: &ContractDefinitionNode,
    ) -> String {
        format!(
            "L{}: The {}",
            source_line,
            match definition_node {
                ContractDefinitionNode::FunctionDefinition(function_definition) => format!(
                    "{} {} in the `{}` {}",
                    function_definition.visibility,
                    if let FunctionKind::Constructor = function_definition.kind {
                        "constructor".to_string()
                    } else {
                        format!(
                            "`{}` {}",
                            function_definition.name, function_definition.kind
                        )
                    },
                    self.name,
                    self.kind,
                ),

                ContractDefinitionNode::ModifierDefinition(modifier_definition) => format!(
                    "`{}` modifier in the `{}` {}",
                    modifier_definition.name, self.name, self.kind,
                ),

                ContractDefinitionNode::UsingForDirective(_) =>
                    format!("`{}` {}", self.name, self.kind,),

                x => panic!("Unsupported definition node: {x:?}"),
            },
        )
    }
}

impl Display for ContractDefinition {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        if let Some(true) = self.is_abstract {
            f.write_str("abstract ")?;
        }

        f.write_fmt(format_args!("{} {}", self.kind, self.name))?;

        for (i, base_contract) in self.base_contracts.iter().enumerate() {
            f.write_fmt(format_args!(
                "{}{}",
                match i {
                    0 => " is ",
                    _ => ", ",
                },
                base_contract
            ))?;
        }

        f.write_str(" {\n")?;

        for node in self.nodes.iter() {
            f.write_fmt(format_args!(
                "\t{}{}\n",
                node,
                match node {
                    ContractDefinitionNode::UsingForDirective(_)
                    | ContractDefinitionNode::EventDefinition(_)
                    | ContractDefinitionNode::ErrorDefinition(_)
                    | ContractDefinitionNode::VariableDeclaration(_)
                    | ContractDefinitionNode::UserDefinedValueTypeDefinition(_) => ";",

                    ContractDefinitionNode::StructDefinition(_)
                    | ContractDefinitionNode::EnumDefinition(_)
                    | ContractDefinitionNode::FunctionDefinition(_)
                    | ContractDefinitionNode::ModifierDefinition(_) => "",
                }
            ))?;
        }

        f.write_str("}")?;

        Ok(())
    }
}
