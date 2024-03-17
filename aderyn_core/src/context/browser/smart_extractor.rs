use std::collections::HashSet;

use crate::{
    ast::*,
    context::workspace_context::ASTNode,
    visitor::ast_visitor::{ASTConstVisitor, Node},
};
use eyre::Result;

pub struct SmartExtractor {
    pub extracted: Vec<ASTNode>,
    pub targets: HashSet<NodeType>,
}

impl SmartExtractor {
    pub fn from<T: Node + ?Sized>(node: &T, targets: HashSet<NodeType>) -> Self {
        let mut extractor = Self {
            extracted: vec![],
            targets,
        };
        node.accept(&mut extractor).unwrap_or_default();
        extractor
    }
}

impl ASTConstVisitor for SmartExtractor {
    fn visit_array_type_name(&mut self, node: &ArrayTypeName) -> Result<bool> {
        if self.targets.contains(&NodeType::ArrayTypeName) {
            self.extracted.push(node.into());
        }
        Ok(true)
    }

    fn visit_assignment(&mut self, node: &Assignment) -> Result<bool> {
        if self.targets.contains(&NodeType::Assignment) {
            self.extracted.push(node.into());
        }
        Ok(true)
    }

    fn visit_binary_operation(&mut self, node: &BinaryOperation) -> Result<bool> {
        if self.targets.contains(&NodeType::BinaryOperation) {
            self.extracted.push(node.into());
        }
        Ok(true)
    }

    fn visit_block(&mut self, node: &Block) -> Result<bool> {
        if self.targets.contains(&NodeType::Block) {
            self.extracted.push(node.into());
        }
        Ok(true)
    }

    fn visit_conditional(&mut self, node: &Conditional) -> Result<bool> {
        if self.targets.contains(&NodeType::Conditional) {
            self.extracted.push(node.into());
        }
        Ok(true)
    }

    fn visit_contract_definition(&mut self, node: &ContractDefinition) -> Result<bool> {
        if self.targets.contains(&NodeType::ContractDefinition) {
            self.extracted.push(node.into());
        }
        Ok(true)
    }

    fn visit_elementary_type_name(&mut self, node: &ElementaryTypeName) -> Result<bool> {
        if self.targets.contains(&NodeType::ElementaryTypeName) {
            self.extracted.push(node.into());
        }
        Ok(true)
    }

    fn visit_elementary_type_name_expression(
        &mut self,
        node: &ElementaryTypeNameExpression,
    ) -> Result<bool> {
        if self
            .targets
            .contains(&NodeType::ElementaryTypeNameExpression)
        {
            self.extracted.push(node.into());
        }
        Ok(true)
    }

    fn visit_emit_statement(&mut self, node: &EmitStatement) -> Result<bool> {
        if self.targets.contains(&NodeType::EmitStatement) {
            self.extracted.push(node.into());
        }
        Ok(true)
    }

    fn visit_enum_definition(&mut self, node: &EnumDefinition) -> Result<bool> {
        if self.targets.contains(&NodeType::EnumDefinition) {
            self.extracted.push(node.into());
        }
        Ok(true)
    }

    fn visit_enum_value(&mut self, node: &EnumValue) -> Result<bool> {
        if self.targets.contains(&NodeType::EnumValue) {
            self.extracted.push(node.into());
        }
        Ok(true)
    }

    fn visit_event_definition(&mut self, node: &EventDefinition) -> Result<bool> {
        if self.targets.contains(&NodeType::EventDefinition) {
            self.extracted.push(node.into());
        }
        Ok(true)
    }

    fn visit_error_definition(&mut self, node: &ErrorDefinition) -> Result<bool> {
        if self.targets.contains(&NodeType::ErrorDefinition) {
            self.extracted.push(node.into());
        }
        Ok(true)
    }

    fn visit_expression_statement(&mut self, node: &ExpressionStatement) -> Result<bool> {
        if self.targets.contains(&NodeType::ExpressionStatement) {
            self.extracted.push(node.into());
        }
        Ok(true)
    }

    fn visit_function_call(&mut self, node: &FunctionCall) -> Result<bool> {
        if self.targets.contains(&NodeType::FunctionCall) {
            self.extracted.push(node.into());
        }
        Ok(true)
    }

    fn visit_function_call_options(&mut self, node: &FunctionCallOptions) -> Result<bool> {
        if self.targets.contains(&NodeType::FunctionCallOptions) {
            self.extracted.push(node.into());
        }
        Ok(true)
    }

    fn visit_function_definition(&mut self, node: &FunctionDefinition) -> Result<bool> {
        if self.targets.contains(&NodeType::FunctionDefinition) {
            self.extracted.push(node.into());
        }
        Ok(true)
    }

    fn visit_function_type_name(&mut self, node: &FunctionTypeName) -> Result<bool> {
        if self.targets.contains(&NodeType::FunctionTypeName) {
            self.extracted.push(node.into());
        }
        Ok(true)
    }

    fn visit_for_statement(&mut self, node: &ForStatement) -> Result<bool> {
        if self.targets.contains(&NodeType::ForStatement) {
            self.extracted.push(node.into());
        }
        Ok(true)
    }

    fn visit_identifier(&mut self, node: &Identifier) -> Result<bool> {
        if self.targets.contains(&NodeType::Identifier) {
            self.extracted.push(node.into());
        }
        Ok(true)
    }

    fn visit_identifier_path(&mut self, node: &IdentifierPath) -> Result<bool> {
        if self.targets.contains(&NodeType::IdentifierPath) {
            self.extracted.push(node.into());
        }
        Ok(true)
    }

    fn visit_if_statement(&mut self, node: &IfStatement) -> Result<bool> {
        if self.targets.contains(&NodeType::IfStatement) {
            self.extracted.push(node.into());
        }
        Ok(true)
    }

    fn visit_import_directive(&mut self, node: &ImportDirective) -> Result<bool> {
        if self.targets.contains(&NodeType::ImportDirective) {
            self.extracted.push(node.into());
        }
        Ok(true)
    }

    fn visit_index_access(&mut self, node: &IndexAccess) -> Result<bool> {
        if self.targets.contains(&NodeType::IndexAccess) {
            self.extracted.push(node.into());
        }
        Ok(true)
    }

    fn visit_index_range_access(&mut self, node: &IndexRangeAccess) -> Result<bool> {
        if self.targets.contains(&NodeType::IndexRangeAccess) {
            self.extracted.push(node.into());
        }
        Ok(true)
    }

    fn visit_inheritance_specifier(&mut self, node: &InheritanceSpecifier) -> Result<bool> {
        if self.targets.contains(&NodeType::InheritanceSpecifier) {
            self.extracted.push(node.into());
        }
        Ok(true)
    }

    fn visit_inline_assembly(&mut self, node: &InlineAssembly) -> Result<bool> {
        if self.targets.contains(&NodeType::InlineAssembly) {
            self.extracted.push(node.into());
        }
        Ok(true)
    }

    fn visit_literal(&mut self, node: &Literal) -> Result<bool> {
        if self.targets.contains(&NodeType::Literal) {
            self.extracted.push(node.into());
        }
        Ok(true)
    }

    fn visit_member_access(&mut self, node: &MemberAccess) -> Result<bool> {
        if self.targets.contains(&NodeType::MemberAccess) {
            self.extracted.push(node.into());
        }
        Ok(true)
    }

    fn visit_new_expression(&mut self, node: &NewExpression) -> Result<bool> {
        if self.targets.contains(&NodeType::NewExpression) {
            self.extracted.push(node.into());
        }
        Ok(true)
    }

    fn visit_mapping(&mut self, node: &Mapping) -> Result<bool> {
        if self.targets.contains(&NodeType::Mapping) {
            self.extracted.push(node.into());
        }
        Ok(true)
    }

    fn visit_modifier_definition(&mut self, node: &ModifierDefinition) -> Result<bool> {
        if self.targets.contains(&NodeType::ModifierDefinition) {
            self.extracted.push(node.into());
        }
        Ok(true)
    }

    fn visit_modifier_invocation(&mut self, node: &ModifierInvocation) -> Result<bool> {
        if self.targets.contains(&NodeType::ModifierInvocation) {
            self.extracted.push(node.into());
        }
        Ok(true)
    }

    fn visit_override_specifier(&mut self, node: &OverrideSpecifier) -> Result<bool> {
        if self.targets.contains(&NodeType::OverrideSpecifier) {
            self.extracted.push(node.into());
        }
        Ok(true)
    }

    fn visit_parameter_list(&mut self, node: &ParameterList) -> Result<bool> {
        if self.targets.contains(&NodeType::ParameterList) {
            self.extracted.push(node.into());
        }
        Ok(true)
    }

    fn visit_pragma_directive(&mut self, node: &PragmaDirective) -> Result<bool> {
        if self.targets.contains(&NodeType::PragmaDirective) {
            self.extracted.push(node.into());
        }
        Ok(true)
    }

    fn visit_return(&mut self, node: &Return) -> Result<bool> {
        if self.targets.contains(&NodeType::Return) {
            self.extracted.push(node.into());
        }
        Ok(true)
    }

    fn visit_revert_statement(&mut self, node: &RevertStatement) -> Result<bool> {
        if self.targets.contains(&NodeType::RevertStatement) {
            self.extracted.push(node.into());
        }
        Ok(true)
    }

    fn visit_source_unit(&mut self, node: &SourceUnit) -> Result<bool> {
        if self.targets.contains(&NodeType::SourceUnit) {
            self.extracted.push(node.into());
        }
        Ok(true)
    }

    fn visit_struct_definition(&mut self, node: &StructDefinition) -> Result<bool> {
        if self.targets.contains(&NodeType::StructDefinition) {
            self.extracted.push(node.into());
        }
        Ok(true)
    }

    fn visit_structured_documentation(&mut self, node: &StructuredDocumentation) -> Result<bool> {
        if self.targets.contains(&NodeType::StructuredDocumentation) {
            self.extracted.push(node.into());
        }
        Ok(true)
    }

    fn visit_try_statement(&mut self, node: &TryStatement) -> Result<bool> {
        if self.targets.contains(&NodeType::TryStatement) {
            self.extracted.push(node.into());
        }
        Ok(true)
    }

    fn visit_try_catch_clause(&mut self, node: &TryCatchClause) -> Result<bool> {
        if self.targets.contains(&NodeType::TryCatchClause) {
            self.extracted.push(node.into());
        }
        Ok(true)
    }

    fn visit_tuple_expression(&mut self, node: &TupleExpression) -> Result<bool> {
        if self.targets.contains(&NodeType::TupleExpression) {
            self.extracted.push(node.into());
        }
        Ok(true)
    }

    fn visit_unary_operation(&mut self, node: &UnaryOperation) -> Result<bool> {
        if self.targets.contains(&NodeType::UnaryOperation) {
            self.extracted.push(node.into());
        }
        Ok(true)
    }

    fn visit_user_defined_type_name(&mut self, node: &UserDefinedTypeName) -> Result<bool> {
        if self.targets.contains(&NodeType::UserDefinedTypeName) {
            self.extracted.push(node.into());
        }
        Ok(true)
    }

    fn visit_user_defined_value_type_definition(
        &mut self,
        node: &UserDefinedValueTypeDefinition,
    ) -> Result<bool> {
        if self
            .targets
            .contains(&NodeType::UserDefinedValueTypeDefinition)
        {
            self.extracted.push(node.into());
        }
        Ok(true)
    }

    fn visit_using_for_directive(&mut self, node: &UsingForDirective) -> Result<bool> {
        if self.targets.contains(&NodeType::UsingForDirective) {
            self.extracted.push(node.into());
        }
        Ok(true)
    }

    fn visit_variable_declaration(&mut self, node: &VariableDeclaration) -> Result<bool> {
        if self.targets.contains(&NodeType::VariableDeclaration) {
            self.extracted.push(node.into());
        }
        Ok(true)
    }

    fn visit_variable_declaration_statement(
        &mut self,
        node: &VariableDeclarationStatement,
    ) -> Result<bool> {
        if self
            .targets
            .contains(&NodeType::VariableDeclarationStatement)
        {
            self.extracted.push(node.into());
        }
        Ok(true)
    }

    fn visit_while_statement(&mut self, node: &WhileStatement) -> Result<bool> {
        if self.targets.contains(&NodeType::WhileStatement) {
            self.extracted.push(node.into());
        }
        Ok(true)
    }
}
