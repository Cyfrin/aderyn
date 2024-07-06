use crate::{ast::*, visitor::ast_visitor::*};
use eyre::Result;

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

impl Node for InheritanceSpecifier {
    fn accept(&self, visitor: &mut impl ASTConstVisitor) -> Result<()> {
        if visitor.visit_inheritance_specifier(self)? {
            match &self.base_name {
                UserDefinedTypeNameOrIdentifierPath::UserDefinedTypeName(type_name) => {
                    type_name.accept(visitor)?
                }
                UserDefinedTypeNameOrIdentifierPath::IdentifierPath(identifier_path) => {
                    identifier_path.accept(visitor)?;
                }
            };
            if self.arguments.is_some() {
                list_accept(self.arguments.as_ref().unwrap(), visitor)?;
            }
        }
        self.accept_metadata(visitor)?;
        visitor.end_visit_inheritance_specifier(self)
    }
    fn accept_metadata(&self, visitor: &mut impl ASTConstVisitor) -> Result<()> {
        if let Some(base_name_id) = self.base_name.get_node_id() {
            visitor.visit_immediate_children(self.id, vec![base_name_id])?;
        }
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
                if let Some(reference_id) = identifier.referenced_declaration {
                    if self.hierarchy_contains_state_variable(source_units, reference_id) {
                        ids.push(reference_id);
                    }
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
                    if let FunctionKind::Constructor = function_definition.kind() {
                        "constructor".to_string()
                    } else {
                        format!(
                            "`{}` {}",
                            function_definition.name,
                            function_definition.kind()
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
