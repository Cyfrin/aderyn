use super::*;
use super::{node::*, *};
use serde::{Deserialize, Serialize};
use std::{collections::HashMap, fmt::Display};

#[derive(Clone, Debug, Deserialize, Serialize, PartialEq, PartialOrd, Eq, Ord, Hash)]
#[serde(rename_all = "lowercase")]
pub enum ContractKind {
    Contract,
    Interface,
    Library,
}

impl Display for ContractKind {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_fmt(format_args!("{}", format!("{:?}", self).to_lowercase()))
    }
}

#[derive(Clone, Debug, Eq, Serialize, PartialEq)]
#[serde(untagged)]
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

impl<'de> Deserialize<'de> for ContractDefinitionNode {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        let json = serde_json::Value::deserialize(deserializer)?;
        let node_type = json.get("nodeType").unwrap().as_str().unwrap();

        match node_type {
            "UsingForDirective" => Ok(ContractDefinitionNode::UsingForDirective(
                serde_json::from_value(json).unwrap(),
            )),
            "StructDefinition" => Ok(ContractDefinitionNode::StructDefinition(
                serde_json::from_value(json).unwrap(),
            )),
            "EnumDefinition" => Ok(ContractDefinitionNode::EnumDefinition(
                serde_json::from_value(json).unwrap(),
            )),
            "VariableDeclaration" => Ok(ContractDefinitionNode::VariableDeclaration(
                serde_json::from_value(json).unwrap(),
            )),
            "EventDefinition" => Ok(ContractDefinitionNode::EventDefinition(
                serde_json::from_value(json).unwrap(),
            )),
            "FunctionDefinition" => Ok(ContractDefinitionNode::FunctionDefinition(
                serde_json::from_value(json).unwrap(),
            )),
            "ModifierDefinition" => Ok(ContractDefinitionNode::ModifierDefinition(
                serde_json::from_value(json).unwrap(),
            )),
            "ErrorDefinition" => Ok(ContractDefinitionNode::ErrorDefinition(
                serde_json::from_value(json).unwrap(),
            )),
            "UserDefinedValueTypeDefinition" => {
                Ok(ContractDefinitionNode::UserDefinedValueTypeDefinition(
                    serde_json::from_value(json).unwrap(),
                ))
            }
            _ => panic!("Invalid contract definition node type: {node_type}"),
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

#[derive(Clone, Debug, Deserialize, Eq, Serialize, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct InheritanceSpecifier {
    pub base_name: IdentifierPath,
    pub arguments: Option<Vec<Expression>>,
    pub src: String,
    pub id: NodeID,
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

#[derive(Clone, Debug, Deserialize, Eq, Serialize, PartialEq)]
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
    pub used_events: Option<Vec<NodeID>>,
    pub used_errors: Option<Vec<NodeID>>,
    pub nodes: Vec<ContractDefinitionNode>,
    pub scope: NodeID,
    pub fully_implemented: Option<bool>,
    pub linearized_base_contracts: Option<Vec<NodeID>>,
    #[serde(rename = "internalFunctionIDs")]
    pub internal_function_ids: Option<HashMap<String, NodeID>>,
    pub src: String,
    pub id: NodeID,
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

    pub fn definition_node_location(&self, definition_node: &ContractDefinitionNode) -> String {
        format!(
            "The {}",
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

#[derive(Debug, PartialEq)]
pub struct ContractDefinitionContext<'a> {
    pub source_units: &'a [SourceUnit],
    pub current_source_unit: &'a SourceUnit,
    pub contract_definition: &'a ContractDefinition,
}

impl<'a> ContractDefinitionContext<'a> {
    pub fn create_using_for_directive_context(
        &self,
        definition_node: &'a ContractDefinitionNode,
        using_for_directive: &'a UsingForDirective,
    ) -> UsingForDirectiveContext<'a> {
        UsingForDirectiveContext {
            source_units: self.source_units,
            current_source_unit: self.current_source_unit,
            contract_definition: self.contract_definition,
            definition_node,
            using_for_directive,
        }
    }

    pub fn create_struct_definition_context(
        &self,
        struct_definition: &'a StructDefinition,
    ) -> StructDefinitionContext<'a> {
        StructDefinitionContext {
            source_units: self.source_units,
            current_source_unit: self.current_source_unit,
            contract_definition: Some(self.contract_definition),
            struct_definition,
        }
    }

    pub fn create_enum_definition_context(
        &self,
        enum_definition: &'a EnumDefinition,
    ) -> EnumDefinitionContext<'a> {
        EnumDefinitionContext {
            source_units: self.source_units,
            current_source_unit: self.current_source_unit,
            contract_definition: Some(self.contract_definition),
            enum_definition,
        }
    }

    pub fn create_variable_declaration_context<'b>(
        &self,
        definition_node: &'a ContractDefinitionNode,
        blocks: &'b mut Vec<&'a Block>,
        variable_declaration: &'a VariableDeclaration,
    ) -> VariableDeclarationContext<'a, 'b> {
        VariableDeclarationContext {
            source_units: self.source_units,
            current_source_unit: self.current_source_unit,
            contract_definition: Some(self.contract_definition),
            definition_node: Some(definition_node),
            blocks: Some(blocks),
            variable_declaration,
        }
    }

    pub fn create_event_definition_context(
        &self,
        event_definition: &'a EventDefinition,
    ) -> EventDefinitionContext<'a> {
        EventDefinitionContext {
            source_units: self.source_units,
            current_source_unit: self.current_source_unit,
            contract_definition: self.contract_definition,
            event_definition,
        }
    }

    pub fn create_function_definition_context(
        &self,
        definition_node: &'a ContractDefinitionNode,
        function_definition: &'a FunctionDefinition,
    ) -> FunctionDefinitionContext<'a> {
        FunctionDefinitionContext {
            source_units: self.source_units,
            current_source_unit: self.current_source_unit,
            contract_definition: self.contract_definition,
            definition_node,
            function_definition,
        }
    }

    pub fn create_modifier_definition_context(
        &self,
        definition_node: &'a ContractDefinitionNode,
        modifier_definition: &'a ModifierDefinition,
    ) -> ModifierDefinitionContext<'a> {
        ModifierDefinitionContext {
            source_units: self.source_units,
            current_source_unit: self.current_source_unit,
            contract_definition: self.contract_definition,
            definition_node,
            modifier_definition,
        }
    }

    pub fn create_error_definition_context(
        &self,
        error_definition: &'a ErrorDefinition,
    ) -> ErrorDefinitionContext<'a> {
        ErrorDefinitionContext {
            source_units: self.source_units,
            current_source_unit: self.current_source_unit,
            contract_definition: Some(self.contract_definition),
            error_definition,
        }
    }

    pub fn create_user_defined_value_type_definition_context(
        &self,
        user_defined_value_type_definition: &'a UserDefinedValueTypeDefinition,
    ) -> UserDefinedValueTypeDefinitionContext<'a> {
        UserDefinedValueTypeDefinitionContext {
            source_units: self.source_units,
            current_source_unit: self.current_source_unit,
            contract_definition: Some(self.contract_definition),
            user_defined_value_type_definition,
        }
    }
}
