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
    macros::accept_id!();
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

    macros::accept_id!();
}
