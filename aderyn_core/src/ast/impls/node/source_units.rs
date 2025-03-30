use crate::{ast::*, visitor::ast_visitor::*};
use eyre::Result;

impl Node for SourceUnitNode {
    fn accept(&self, visitor: &mut impl ASTConstVisitor) -> Result<()> {
        match self {
            SourceUnitNode::PragmaDirective(pragma_directive) => pragma_directive.accept(visitor),
            SourceUnitNode::ImportDirective(import_directive) => import_directive.accept(visitor),
            SourceUnitNode::ContractDefinition(contract_definition) => {
                contract_definition.accept(visitor)
            }
            SourceUnitNode::StructDefinition(struct_definition) => {
                struct_definition.accept(visitor)
            }
            SourceUnitNode::EnumDefinition(enum_definition) => enum_definition.accept(visitor),
            SourceUnitNode::ErrorDefinition(error_definition) => error_definition.accept(visitor),
            SourceUnitNode::VariableDeclaration(variable_declaration) => {
                variable_declaration.accept(visitor)
            }
            SourceUnitNode::UserDefinedValueTypeDefinition(user_defined_value_type_definition) => {
                user_defined_value_type_definition.accept(visitor)
            }
            SourceUnitNode::FunctionDefinition(function_defn) => function_defn.accept(visitor),
            SourceUnitNode::UsingForDirective(using_for_directive) => {
                using_for_directive.accept(visitor)
            }
            SourceUnitNode::EventDefinition(event_definition) => event_definition.accept(visitor),
        }
    }
    fn accept_id(&self, visitor: &mut impl ASTConstVisitor) -> Result<()> {
        visitor.visit_node_id(self.get_node_id())?;
        Ok(())
    }
}

impl Node for SourceUnit {
    fn accept(&self, visitor: &mut impl ASTConstVisitor) -> Result<()> {
        if visitor.visit_source_unit(self)? {
            list_accept(&self.nodes, visitor)?;
        }
        self.accept_metadata(visitor)?;
        visitor.end_visit_source_unit(self)
    }
    fn accept_metadata(&self, visitor: &mut impl ASTConstVisitor) -> Result<()> {
        let node_ids = &self.nodes.iter().flat_map(|x| x.get_node_id()).collect::<Vec<_>>();
        visitor.visit_immediate_children(self.id, node_ids.clone())?;
        Ok(())
    }
    macros::accept_id!();
}
