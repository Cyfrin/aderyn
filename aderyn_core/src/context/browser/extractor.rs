use crate::{
    ast::*,
    context::workspace::WorkspaceContext,
    visitor::ast_visitor::{ASTConstVisitor, Node},
};
use eyre::Result;

use super::macros::generate_extraction_library;

////////// PUBLICLY AVAILABLE EXTRACTION LIBRARY /////////////////////////

generate_extraction_library! {
    ExtractArrayTypeNames | visit_array_type_name => ArrayTypeName |,
    ExtractAssignments | visit_assignment => Assignment |,
    ExtractBinaryOperations | visit_binary_operation => BinaryOperation |,
    ExtractBlocks | visit_block => Block |,
    ExtractConditionals | visit_conditional => Conditional |,
    ExtractContractDefinitions | visit_contract_definition => ContractDefinition |,
    ExtractElementaryTypeNames | visit_elementary_type_name => ElementaryTypeName |,
    ExtractEmitStatements | visit_emit_statement => EmitStatement |,
    ExtractEnumDefinitions | visit_enum_definition => EnumDefinition |,
    ExtractEnumValues | visit_enum_value => EnumValue |,
    ExtractEventDefinitions | visit_event_definition => EventDefinition |,
    ExtractErrorDefinitions | visit_error_definition => ErrorDefinition |,
    ExtractExpressionStatements | visit_expression_statement => ExpressionStatement |,
    ExtractFunctionCalls | visit_function_call => FunctionCall |,
    ExtractFunctionCallOptions | visit_function_call_options => FunctionCallOptions |,
    ExtractFunctionDefinitions | visit_function_definition => FunctionDefinition |,
    ExtractFunctionTypeNames | visit_function_type_name => FunctionTypeName |,
    ExtractForStatements | visit_for_statement => ForStatement |,
    ExtractIdentifiers | visit_identifier => Identifier |,
    ExtractIdentifierPaths | visit_identifier_path => IdentifierPath |,
    ExtractIfStatements | visit_if_statement => IfStatement |,
    ExtractImportDirectives | visit_import_directive => ImportDirective |,
    ExtractIndexAccesses | visit_index_access => IndexAccess |,
    ExtractIndexRangeAccesses | visit_index_range_access => IndexRangeAccess |,
    ExtractInheritanceSpecifiers | visit_inheritance_specifier => InheritanceSpecifier |,
    ExtractInlineAssemblies | visit_inline_assembly => InlineAssembly |,
    ExtractLiterals | visit_literal => Literal |,
    ExtractMemberAccesses | visit_member_access => MemberAccess |,
    ExtractNewExpressions | visit_new_expression => NewExpression |,
    ExtractMappings | visit_mapping => Mapping |,
    ExtractModifierDefinitions | visit_modifier_definition => ModifierDefinition |,
    ExtractModifierInvocations | visit_modifier_invocation => ModifierInvocation |,
    ExtractOverrideSpecifiers | visit_override_specifier => OverrideSpecifier |,
    ExtractParameterLists | visit_parameter_list => ParameterList |,
    ExtractPragmaDirectives | visit_pragma_directive => PragmaDirective |,
    ExtractReturns | visit_return => Return |,
    ExtractRevertStatements | visit_revert_statement => RevertStatement |,
    ExtractStructDefinitions | visit_struct_definition => StructDefinition |,
    ExtractStructuredDocumentations | visit_structured_documentation => StructuredDocumentation |,
    ExtractTryStatements | visit_try_statement => TryStatement |,
    ExtractTryCatchClauses | visit_try_catch_clause => TryCatchClause |,
    ExtractTupleExpressions | visit_tuple_expression => TupleExpression |,
    ExtractUnaryOperations | visit_unary_operation => UnaryOperation |,
    ExtractUserDefinedTypeNames | visit_user_defined_type_name => UserDefinedTypeName |,
    ExtractUsingForDirectives | visit_using_for_directive => UsingForDirective |,
    ExtractVariableDeclarations | visit_variable_declaration => VariableDeclaration |,
    ExtractWhileStatements | visit_while_statement => WhileStatement |,
    ExtractDoWhileStatements | visit_do_while_statement => DoWhileStatement |,
    ExtractBreakStatements | visit_break_statement => Break |,
    ExtractContinueStatements | visit_continue_statement => Continue |,
    ExtractPlaceholderStatements | visit_placeholder_statement => PlaceholderStatement|,
}

/////////// EXTRACTION UTILS FOR CRATE - LEVEL ACCESS //////////////////

// ExtractImmediateChildren is an extractor that extracts immediate children from a node
#[derive(Default)]
pub(crate) struct ExtractImmediateChildrenIDs {
    pub extracted: Vec<NodeID>,
}

impl ExtractImmediateChildrenIDs {
    pub(crate) fn from<T: Node + ?Sized>(node: &T) -> Self {
        let mut extractor: ExtractImmediateChildrenIDs = Self::default();
        node.accept_metadata(&mut extractor).unwrap_or_default();
        extractor
    }
}

impl ASTConstVisitor for ExtractImmediateChildrenIDs {
    fn visit_immediate_children(
        &mut self,
        _node_id: NodeID,
        node_children_ids: Vec<NodeID>,
    ) -> Result<()> {
        self.extracted.extend(node_children_ids);
        Ok(())
    }
}

// Extract Reference Declaration IDs
#[derive(Default)]
pub struct ExtractReferencedDeclarations {
    pub extracted: Vec<NodeID>,
}

impl ExtractReferencedDeclarations {
    pub fn from<T: Node + ?Sized>(node: &T) -> Self {
        let mut extractor: ExtractReferencedDeclarations = Self::default();
        node.accept(&mut extractor).unwrap_or_default();
        extractor
    }
}

impl ASTConstVisitor for ExtractReferencedDeclarations {
    fn visit_member_access(&mut self, node: &MemberAccess) -> Result<bool> {
        if let Some(referenced_id) = node.referenced_declaration {
            self.extracted.push(referenced_id);
        }
        Ok(true)
    }
    fn visit_identifier(&mut self, node: &Identifier) -> Result<bool> {
        if let Some(referenced_id) = node.referenced_declaration {
            self.extracted.push(referenced_id);
        }
        Ok(true)
    }
    fn visit_identifier_path(&mut self, node: &IdentifierPath) -> Result<bool> {
        self.extracted.push(node.referenced_declaration);
        Ok(true)
    }
    fn visit_user_defined_type_name(&mut self, node: &UserDefinedTypeName) -> Result<bool> {
        self.extracted.push(node.referenced_declaration);
        Ok(true)
    }
}

// Extract Reference Declaration IDs
pub struct ExtractReferencedDeclarationsConditionally<'a> {
    pub extracted: Vec<NodeID>,
    pub condition: Box<dyn Fn(NodeID, &'a WorkspaceContext) -> bool>,
    pub context: &'a WorkspaceContext,
}

impl<'a> ExtractReferencedDeclarationsConditionally<'a> {
    pub fn from<T: Node + ?Sized>(
        node: &T,
        context: &'a WorkspaceContext,
        condition: Box<dyn Fn(NodeID, &'a WorkspaceContext) -> bool>,
    ) -> Self {
        let mut extractor: ExtractReferencedDeclarationsConditionally =
            ExtractReferencedDeclarationsConditionally { extracted: vec![], condition, context };
        node.accept(&mut extractor).unwrap_or_default();
        extractor
    }
}

impl ASTConstVisitor for ExtractReferencedDeclarationsConditionally<'_> {
    fn visit_member_access(&mut self, node: &MemberAccess) -> Result<bool> {
        if !self.condition.as_ref()(node.id, self.context) {
            return Ok(true);
        }
        if let Some(referenced_id) = node.referenced_declaration {
            self.extracted.push(referenced_id);
        }
        Ok(true)
    }
    fn visit_identifier(&mut self, node: &Identifier) -> Result<bool> {
        if !self.condition.as_ref()(node.id, self.context) {
            return Ok(true);
        }
        if let Some(referenced_id) = node.referenced_declaration {
            self.extracted.push(referenced_id);
        }
        Ok(true)
    }
    fn visit_identifier_path(&mut self, node: &IdentifierPath) -> Result<bool> {
        if !self.condition.as_ref()(node.id, self.context) {
            return Ok(true);
        }
        self.extracted.push(node.referenced_declaration);
        Ok(true)
    }
    fn visit_user_defined_type_name(&mut self, node: &UserDefinedTypeName) -> Result<bool> {
        if !self.condition.as_ref()(node.id, self.context) {
            return Ok(true);
        }
        self.extracted.push(node.referenced_declaration);
        Ok(true)
    }
}
