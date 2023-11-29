use eyre::Result;
use std::collections::HashMap;
use tokei::Language;

use crate::ast::*;

#[derive(Debug, Clone, PartialEq)]
pub enum ASTNode<'a, 'b, 'c> {
    SourceUnitContext(SourceUnitContext<'a>),
    PragmaDirectiveContext(PragmaDirectiveContext<'a>),
    ImportDirectiveContext(ImportDirectiveContext<'a>),
    StructDefinitionContext(StructDefinitionContext<'a>),
    EnumDefinitionContext(EnumDefinitionContext<'a>),
    ContractDefinitionContext(ContractDefinitionContext<'a>),
    UsingForDirectiveContext(UsingForDirectiveContext<'a>),
    VariableDeclarationContext(VariableDeclarationContext<'a, 'b>),
    EventDefinitionContext(EventDefinitionContext<'a>),
    ErrorDefinitionContext(ErrorDefinitionContext<'a>),
    ModifierDefinitionContext(ModifierDefinitionContext<'a>),
    UserDefinedValueTypeDefinitionContext(UserDefinedValueTypeDefinitionContext<'a>),
    FunctionDefinitionContext(FunctionDefinitionContext<'a>),
    ModifierInvocationContext(ModifierInvocationContext<'a>),
    BlockContext(BlockContext<'a, 'b>),
    StatementContext(StatementContext<'a, 'b>),
    IfStatementContext(IfStatementContext<'a, 'b>),
    ForStatementContext(ForStatementContext<'a, 'b>),
    WhileStatementContext(WhileStatementContext<'a, 'b>),
    DoWhileStatementContext(DoWhileStatementContext<'a, 'b>),
    EmitStatementContext(EmitStatementContext<'a, 'b>),
    TryStatementContext(TryStatementContext<'a, 'b>),
    RevertStatementContext(RevertStatementContext<'a, 'b>),
    BlockOrStatementContext(BlockOrStatementContext<'a, 'b>),
    ReturnContext(ReturnContext<'a, 'b>),
    ExpressionContext(ExpressionContext<'a, 'b>),
    LiteralContext(LiteralContext<'a, 'b>),
    IdentifierContext(IdentifierContext<'a, 'b>),
    UnaryOperationContext(UnaryOperationContext<'a, 'b>),
    BinaryOperationContext(BinaryOperationContext<'a, 'b>),
    ConditionalContext(ConditionalContext<'a, 'b>),
    AssignmentContext(AssignmentContext<'a, 'b>),
    FunctionCallContext(FunctionCallContext<'a, 'b>),
    FunctionCallOptionsContext(FunctionCallOptionsContext<'a, 'b>),
    IndexAccessContext(IndexAccessContext<'a, 'b>),
    IndexRangeAccessContext(IndexRangeAccessContext<'a, 'b>),
    MemberAccessContext(MemberAccessContext<'a, 'b>),
    ElementaryTypeNameExpressionContext(ElementaryTypeNameExpressionContext<'a, 'b>),
    TupleExpressionContext(TupleExpressionContext<'a, 'b>),
    NewExpressionContext(NewExpressionContext<'a, 'b>),
    InlineAssemblyContext(InlineAssemblyContext<'a, 'b>),
    YulBlockContext(YulBlockContext<'a, 'b, 'c>),
    YulStatementContext(YulStatementContext<'a, 'b, 'c>),
    YulIfContext(YulIfContext<'a, 'b, 'c>),
    YulSwitchContext(YulSwitchContext<'a, 'b, 'c>),
    YulForLoopContext(YulForLoopContext<'a, 'b, 'c>),
    YulCaseContext(YulCaseContext<'a, 'b, 'c>),
    YulAssignmentContext(YulAssignmentContext<'a, 'b, 'c>),
    YulVariableDeclarationContext(YulVariableDeclarationContext<'a, 'b, 'c>),
    YulExpressionStatementContext(YulExpressionStatementContext<'a, 'b, 'c>),
    YulFunctionDefinitionContext(YulFunctionDefinitionContext<'a, 'b, 'c>),
    YulTypedNameContext(YulTypedNameContext<'a, 'b, 'c>),
    YulExpressionContext(YulExpressionContext<'a, 'b, 'c>),
    YulLiteralContext(YulLiteralContext<'a, 'b, 'c>),
    YulIdentifierContext(YulIdentifierContext<'a, 'b, 'c>),
    YulFunctionCallContext(YulFunctionCallContext<'a, 'b, 'c>),
}

impl<'a, 'b, 'c> ASTNode<'a, 'b, 'c> {
    pub fn src(&self) -> Option<&str> {
        match self {
            ASTNode::SourceUnitContext(node) => Some(&node.src),
            ASTNode::PragmaDirectiveContext(node) => Some(&node.src),
            ASTNode::ImportDirectiveContext(node) => Some(&node.src),
            ASTNode::StructDefinitionContext(node) => Some(&node.src),
            ASTNode::EnumDefinitionContext(node) => Some(&node.src),
            ASTNode::ContractDefinitionContext(node) => Some(&node.src),
            ASTNode::UsingForDirectiveContext(node) => Some(&node.src),
            ASTNode::VariableDeclarationContext(node) => Some(&node.src),
            ASTNode::EventDefinitionContext(node) => Some(&node.src),
            ASTNode::ErrorDefinitionContext(node) => Some(&node.src),
            ASTNode::ModifierDefinitionContext(node) => Some(&node.src),
            ASTNode::UserDefinedValueTypeDefinitionContext(node) => Some(&node.src),
            ASTNode::FunctionDefinitionContext(node) => Some(&node.src),
            ASTNode::ModifierInvocationContext(node) => Some(&node.src),
            ASTNode::BlockContext(node) => Some(&node.src),
            ASTNode::StatementContext(node) => Some(&node.src),
            ASTNode::IfStatementContext(node) => Some(&node.src),
            ASTNode::ForStatementContext(node) => Some(&node.src),
            ASTNode::WhileStatementContext(node) => Some(&node.src),
            ASTNode::DoWhileStatementContext(node) => Some(&node.src),
            ASTNode::EmitStatementContext(node) => Some(&node.src),
            ASTNode::TryStatementContext(node) => Some(&node.src),
            ASTNode::RevertStatementContext(node) => Some(&node.src),
            ASTNode::BlockOrStatementContext(node) => Some(&node.src),
            ASTNode::ReturnContext(node) => Some(&node.src),
            ASTNode::ExpressionContext(node) => Some(&node.src),
            ASTNode::LiteralContext(node) => Some(&node.src),
            ASTNode::IdentifierContext(node) => Some(&node.src),
            ASTNode::UnaryOperationContext(node) => Some(&node.src),
            ASTNode::BinaryOperationContext(node) => Some(&node.src),
            ASTNode::ConditionalContext(node) => Some(&node.src),
            ASTNode::AssignmentContext(node) => Some(&node.src),
            ASTNode::FunctionCallContext(node) => Some(&node.src),
            ASTNode::FunctionCallOptionsContext(node) => Some(&node.src),
            ASTNode::IndexAccessContext(node) => Some(&node.src),
            ASTNode::IndexRangeAccessContext(node) => Some(&node.src),
            ASTNode::MemberAccessContext(node) => Some(&node.src),
            ASTNode::ElementaryTypeNameExpressionContext(node) => Some(&node.src),
            ASTNode::TupleExpressionContext(node) => Some(&node.src),
            ASTNode::NewExpressionContext(node) => Some(&node.src),
            ASTNode::InlineAssemblyContext(node) => Some(&node.src),
            ASTNode::YulBlockContext(node) => Some(&node.src),
            ASTNode::YulStatementContext(node) => Some(&node.src),
            ASTNode::YulIfContext(node) => todo!(),
            ASTNode::YulSwitchContext(node) => todo!(),
            ASTNode::YulForLoopContext(node) => todo!(),
            ASTNode::YulCaseContext(node) => todo!(),
            ASTNode::YulAssignmentContext(node) => todo!(),
            ASTNode::YulVariableDeclarationContext(node) => todo!(),
            ASTNode::YulExpressionStatementContext(node) => todo!(),
            ASTNode::YulFunctionDefinitionContext(node) => todo!(),
            ASTNode::YulTypedNameContext(node) => todo!(),
            ASTNode::YulExpressionContext(node) => todo!(),
            ASTNode::YulLiteralContext(node) => todo!(),
            ASTNode::YulIdentifierContext(node) => todo!(),
            ASTNode::YulFunctionCallContext(node) => todo!(),
        }
    }
}

#[derive(Default, Debug)]
pub struct ContextLoader<'a, 'b, 'c> {
    sloc_stats: Language,
    pub nodes: HashMap<i64, ASTNode<'a, 'b, 'c>>,

    // Vectors of all context nodes, per type
    pub source_units: Vec<SourceUnitContext<'a>>,
    pub pragma_directives: Vec<PragmaDirectiveContext<'a>>,
    pub import_directives: Vec<ImportDirectiveContext<'a>>,
    pub struct_definitions: Vec<StructDefinitionContext<'a>>,
    pub enum_definitions: Vec<EnumDefinitionContext<'a>>,
    pub contract_definitions: Vec<ContractDefinitionContext<'a>>,
    pub using_for_directives: Vec<UsingForDirectiveContext<'a>>,
    pub variable_declarations: Vec<VariableDeclarationContext<'a, 'b>>,
    pub event_definitions: Vec<EventDefinitionContext<'a>>,
    pub error_definitions: Vec<ErrorDefinitionContext<'a>>,
    pub modifier_definitions: Vec<ModifierDefinitionContext<'a>>,
    pub user_defined_value_type_definitions: Vec<UserDefinedValueTypeDefinitionContext<'a>>,
    pub function_definitions: Vec<FunctionDefinitionContext<'a>>,
    pub modifier_invocations: Vec<ModifierInvocationContext<'a>>,
    pub blocks: Vec<BlockContext<'a, 'b>>,
    pub statements: Vec<StatementContext<'a, 'b>>,
    pub if_statements: Vec<IfStatementContext<'a, 'b>>,
    pub for_statements: Vec<ForStatementContext<'a, 'b>>,
    pub while_statements: Vec<WhileStatementContext<'a, 'b>>,
    pub do_while_statements: Vec<DoWhileStatementContext<'a, 'b>>,
    pub emit_statements: Vec<EmitStatementContext<'a, 'b>>,
    pub try_statements: Vec<TryStatementContext<'a, 'b>>,
    pub revert_statements: Vec<RevertStatementContext<'a, 'b>>,
    pub block_or_statements: Vec<BlockOrStatementContext<'a, 'b>>,
    pub returns: Vec<ReturnContext<'a, 'b>>,
    pub expressions: Vec<ExpressionContext<'a, 'b>>,
    pub literals: Vec<LiteralContext<'a, 'b>>,
    pub identifiers: Vec<IdentifierContext<'a, 'b>>,
    pub unary_operations: Vec<UnaryOperationContext<'a, 'b>>,
    pub binary_operations: Vec<BinaryOperationContext<'a, 'b>>,
    pub conditionals: Vec<ConditionalContext<'a, 'b>>,
    pub assignments: Vec<AssignmentContext<'a, 'b>>,
    pub function_calls: Vec<FunctionCallContext<'a, 'b>>,
    pub function_call_options: Vec<FunctionCallOptionsContext<'a, 'b>>,
    pub index_accesses: Vec<IndexAccessContext<'a, 'b>>,
    pub index_range_accesses: Vec<IndexRangeAccessContext<'a, 'b>>,
    pub member_accesses: Vec<MemberAccessContext<'a, 'b>>,
    pub elementary_type_name_expressions: Vec<ElementaryTypeNameExpressionContext<'a, 'b>>,
    pub tuple_expressions: Vec<TupleExpressionContext<'a, 'b>>,
    pub new_expressions: Vec<NewExpressionContext<'a, 'b>>,
    pub inline_assemblies: Vec<InlineAssemblyContext<'a, 'b>>,
    pub yul_blocks: Vec<YulBlockContext<'a, 'b, 'c>>,
    pub yul_statements: Vec<YulStatementContext<'a, 'b, 'c>>,
    pub yul_ifs: Vec<YulIfContext<'a, 'b, 'c>>,
    pub yul_switches: Vec<YulSwitchContext<'a, 'b, 'c>>,
    pub yul_for_loops: Vec<YulForLoopContext<'a, 'b, 'c>>,
    pub yul_cases: Vec<YulCaseContext<'a, 'b, 'c>>,
    pub yul_assignments: Vec<YulAssignmentContext<'a, 'b, 'c>>,
    pub yul_variable_declarations: Vec<YulVariableDeclarationContext<'a, 'b, 'c>>,
    pub yul_expression_statements: Vec<YulExpressionStatementContext<'a, 'b, 'c>>,
    pub yul_function_definitions: Vec<YulFunctionDefinitionContext<'a, 'b, 'c>>,
    pub yul_typed_names: Vec<YulTypedNameContext<'a, 'b, 'c>>,
    pub yul_expressions: Vec<YulExpressionContext<'a, 'b, 'c>>,
    pub yul_literals: Vec<YulLiteralContext<'a, 'b, 'c>>,
    pub yul_identifiers: Vec<YulIdentifierContext<'a, 'b, 'c>>,
    pub yul_function_calls: Vec<YulFunctionCallContext<'a, 'b, 'c>>,
}

impl<'a, 'b, 'c> ContextLoader<'a, 'b, 'c> {
    // SETTERS

    pub fn set_sloc_stats(&mut self, sloc_stats: Language) {
        self.sloc_stats = sloc_stats;
    }

    pub fn set_source_unit_source_content(&mut self, id: i64, source: String) {
        if let Some(source_unit) = self.source_units.iter_mut().find(|unit| unit.id == id) {
            source_unit.source = Some(source);
        }
    }

    // GETTERS

    pub fn get_sloc_stats(&self) -> &Language {
        &self.sloc_stats
    }

    pub fn get_node(&self, id: i64) -> Option<&ASTNode> {
        self.nodes.get(&id)
    }

    pub fn get_array_type_names(&self) -> Vec<&ArrayTypeName> {
        self.array_type_names.keys().collect()
    }

    pub fn get_assignments(&self) -> Vec<&Assignment> {
        self.assignments.keys().collect()
    }

    pub fn get_binary_operations(&self) -> Vec<&BinaryOperation> {
        self.binary_operations.keys().collect()
    }

    pub fn get_blocks(&self) -> Vec<&Block> {
        self.blocks.keys().collect()
    }

    pub fn get_conditionals(&self) -> Vec<&Conditional> {
        self.conditionals.keys().collect()
    }

    pub fn get_contract_definitions(&self) -> Vec<&ContractDefinition> {
        self.contract_definitions.keys().collect()
    }

    pub fn get_elementary_type_names(&self) -> Vec<&ElementaryTypeName> {
        self.elementary_type_names.keys().collect()
    }

    pub fn get_elementary_type_name_expressions(&self) -> Vec<&ElementaryTypeNameExpression> {
        self.elementary_type_name_expressions.keys().collect()
    }

    pub fn get_emit_statements(&self) -> Vec<&EmitStatement> {
        self.emit_statements.keys().collect()
    }

    pub fn get_enum_definitions(&self) -> Vec<&EnumDefinition> {
        self.enum_definitions.keys().collect()
    }

    pub fn get_enum_values(&self) -> Vec<&EnumValue> {
        self.enum_values.keys().collect()
    }

    pub fn get_event_definitions(&self) -> Vec<&EventDefinition> {
        self.event_definitions.keys().collect()
    }

    pub fn get_error_definitions(&self) -> Vec<&ErrorDefinition> {
        self.error_definitions.keys().collect()
    }

    pub fn get_expression_statements(&self) -> Vec<&ExpressionStatement> {
        self.expression_statements.keys().collect()
    }

    pub fn get_function_calls(&self) -> Vec<&FunctionCall> {
        self.function_calls.keys().collect()
    }

    pub fn get_function_call_options(&self) -> Vec<&FunctionCallOptions> {
        self.function_call_options.keys().collect()
    }

    pub fn get_function_definitions(&self) -> Vec<&FunctionDefinition> {
        self.function_definitions.keys().collect()
    }

    pub fn get_function_type_names(&self) -> Vec<&FunctionTypeName> {
        self.function_type_names.keys().collect()
    }

    pub fn get_for_statements(&self) -> Vec<&ForStatement> {
        self.for_statements.keys().collect()
    }

    pub fn get_identifiers(&self) -> Vec<&Identifier> {
        self.identifiers.keys().collect()
    }

    pub fn get_identifier_paths(&self) -> Vec<&IdentifierPath> {
        self.identifier_paths.keys().collect()
    }

    pub fn get_if_statements(&self) -> Vec<&IfStatement> {
        self.if_statements.keys().collect()
    }

    pub fn get_import_directives(&self) -> Vec<&ImportDirective> {
        self.import_directives.keys().collect()
    }

    pub fn get_index_accesses(&self) -> Vec<&IndexAccess> {
        self.index_accesses.keys().collect()
    }

    pub fn get_index_range_accesses(&self) -> Vec<&IndexRangeAccess> {
        self.index_range_accesses.keys().collect()
    }

    pub fn get_inheritance_specifiers(&self) -> Vec<&InheritanceSpecifier> {
        self.inheritance_specifiers.keys().collect()
    }

    pub fn get_inline_assemblies(&self) -> Vec<&InlineAssembly> {
        self.inline_assemblies.keys().collect()
    }

    pub fn get_literals(&self) -> Vec<&Literal> {
        self.literals.keys().collect()
    }

    pub fn get_member_accesses(&self) -> Vec<&MemberAccess> {
        self.member_accesses.keys().collect()
    }

    pub fn get_new_expressions(&self) -> Vec<&NewExpression> {
        self.new_expressions.keys().collect()
    }

    pub fn get_mappings(&self) -> Vec<&Mapping> {
        self.mappings.keys().collect()
    }

    pub fn get_modifier_definitions(&self) -> Vec<&ModifierDefinition> {
        self.modifier_definitions.keys().collect()
    }

    pub fn get_modifier_invocations(&self) -> Vec<&ModifierInvocation> {
        self.modifier_invocations.keys().collect()
    }

    pub fn get_override_specifiers(&self) -> Vec<&OverrideSpecifier> {
        self.override_specifiers.keys().collect()
    }

    pub fn get_parameter_lists(&self) -> Vec<&ParameterList> {
        self.parameter_lists.keys().collect()
    }

    pub fn get_pragma_directives(&self) -> Vec<&PragmaDirective> {
        self.pragma_directives.keys().collect()
    }

    pub fn get_returns(&self) -> Vec<&Return> {
        self.returns.keys().collect()
    }

    pub fn get_revert_statements(&self) -> Vec<&RevertStatement> {
        self.revert_statements.keys().collect()
    }

    pub fn get_struct_definitions(&self) -> Vec<&StructDefinition> {
        self.struct_definitions.keys().collect()
    }

    pub fn get_structured_documentations(&self) -> Vec<&StructuredDocumentation> {
        self.structured_documentations.keys().collect()
    }

    pub fn get_try_statements(&self) -> Vec<&TryStatement> {
        self.try_statements.keys().collect()
    }

    pub fn get_try_catch_clauses(&self) -> Vec<&TryCatchClause> {
        self.try_catch_clauses.keys().collect()
    }

    pub fn get_tuple_expressions(&self) -> Vec<&TupleExpression> {
        self.tuple_expressions.keys().collect()
    }

    pub fn get_unary_operations(&self) -> Vec<&UnaryOperation> {
        self.unary_operations.keys().collect()
    }

    pub fn get_user_defined_type_names(&self) -> Vec<&UserDefinedTypeName> {
        self.user_defined_type_names.keys().collect()
    }

    pub fn get_user_defined_value_type_definitions(&self) -> Vec<&UserDefinedValueTypeDefinition> {
        self.user_defined_value_type_definitions.keys().collect()
    }

    pub fn get_using_for_directives(&self) -> Vec<&UsingForDirective> {
        self.using_for_directives.keys().collect()
    }

    pub fn get_variable_declarations(&self) -> Vec<&VariableDeclaration> {
        self.variable_declarations.keys().collect()
    }

    pub fn get_variable_declaration_statements(&self) -> Vec<&VariableDeclarationStatement> {
        self.variable_declaration_statements.keys().collect()
    }

    pub fn get_while_statements(&self) -> Vec<&WhileStatement> {
        self.while_statements.keys().collect()
    }

    pub fn get_source_units(&self) -> Vec<&SourceUnit> {
        self.source_units.iter().collect()
    }

    pub fn get_source_unit_from_child_node(&self, node: &ASTNode) -> Option<&SourceUnit> {
        let source_unit_id = match node {
            ASTNode::ArrayTypeName(node) => self.array_type_names.get(node),
            ASTNode::Assignment(node) => self.assignments.get(node),
            ASTNode::BinaryOperation(node) => self.binary_operations.get(node),
            ASTNode::Block(node) => self.blocks.get(node),
            ASTNode::Conditional(node) => self.conditionals.get(node),
            ASTNode::ContractDefinition(node) => self.contract_definitions.get(node),
            ASTNode::ElementaryTypeName(node) => self.elementary_type_names.get(node),
            ASTNode::ElementaryTypeNameExpression(node) => {
                self.elementary_type_name_expressions.get(node)
            }
            ASTNode::EmitStatement(node) => self.emit_statements.get(node),
            ASTNode::EnumDefinition(node) => self.enum_definitions.get(node),
            ASTNode::EnumValue(node) => self.enum_values.get(node),
            ASTNode::EventDefinition(node) => self.event_definitions.get(node),
            ASTNode::ErrorDefinition(node) => self.error_definitions.get(node),
            ASTNode::ExpressionStatement(node) => self.expression_statements.get(node),
            ASTNode::FunctionCall(node) => self.function_calls.get(node),
            ASTNode::FunctionCallOptions(node) => self.function_call_options.get(node),
            ASTNode::FunctionDefinition(node) => self.function_definitions.get(node),
            ASTNode::FunctionTypeName(node) => self.function_type_names.get(node),
            ASTNode::ForStatement(node) => self.for_statements.get(node),
            ASTNode::Identifier(node) => self.identifiers.get(node),
            ASTNode::IdentifierPath(node) => self.identifier_paths.get(node),
            ASTNode::IfStatement(node) => self.if_statements.get(node),
            ASTNode::ImportDirective(node) => self.import_directives.get(node),
            ASTNode::IndexAccess(node) => self.index_accesses.get(node),
            ASTNode::IndexRangeAccess(node) => self.index_range_accesses.get(node),
            ASTNode::InheritanceSpecifier(node) => self.inheritance_specifiers.get(node),
            ASTNode::InlineAssembly(node) => self.inline_assemblies.get(node),
            ASTNode::Literal(node) => self.literals.get(node),
            ASTNode::MemberAccess(node) => self.member_accesses.get(node),
            ASTNode::NewExpression(node) => self.new_expressions.get(node),
            ASTNode::Mapping(node) => self.mappings.get(node),
            ASTNode::ModifierDefinition(node) => self.modifier_definitions.get(node),
            ASTNode::ModifierInvocation(node) => self.modifier_invocations.get(node),
            ASTNode::OverrideSpecifier(node) => self.override_specifiers.get(node),
            ASTNode::ParameterList(node) => self.parameter_lists.get(node),
            ASTNode::PragmaDirective(node) => self.pragma_directives.get(node),
            ASTNode::Return(node) => self.returns.get(node),
            ASTNode::RevertStatement(node) => self.revert_statements.get(node),
            ASTNode::SourceUnit(node) => Some(&node.id),
            ASTNode::StructDefinition(node) => self.struct_definitions.get(node),
            ASTNode::StructuredDocumentation(node) => self.structured_documentations.get(node),
            ASTNode::TryStatement(node) => self.try_statements.get(node),
            ASTNode::TryCatchClause(node) => self.try_catch_clauses.get(node),
            ASTNode::TupleExpression(node) => self.tuple_expressions.get(node),
            ASTNode::UnaryOperation(node) => self.unary_operations.get(node),
            ASTNode::UserDefinedTypeName(node) => self.user_defined_type_names.get(node),
            ASTNode::UserDefinedValueTypeDefinition(node) => {
                self.user_defined_value_type_definitions.get(node)
            }
            ASTNode::UsingForDirective(node) => self.using_for_directives.get(node),
            ASTNode::VariableDeclaration(node) => self.variable_declarations.get(node),
            ASTNode::VariableDeclarationStatement(node) => {
                self.variable_declaration_statements.get(node)
            }
            ASTNode::WhileStatement(node) => self.while_statements.get(node),
        };

        // iterate through self.source_units until the source unit with the id matching `source_unit_id` is found, then return its `absolute_path`

        source_unit_id.and_then(|&id| {
            self.source_units
                .iter()
                .find(|source_unit| source_unit.id == id)
        })
    }

    pub fn get_source_unit_contract_path_from(&self, node: &ASTNode) -> Option<&String> {
        let source_unit = self.get_source_unit_from_child_node(node);
        source_unit.and_then(|source_unit| source_unit.absolute_path.as_ref())
    }

    pub fn get_node_sort_key(&self, node: &ASTNode) -> (String, usize) {
        let source_unit = self.get_source_unit_from_child_node(node).unwrap();
        let absolute_path = source_unit.absolute_path.as_ref().unwrap().clone();
        let source_line = node
            .src()
            .map(|src| source_unit.source_line(src).unwrap_or(0)) // If `src` is `Some`, get the line number, else return 0
            .unwrap_or(0); // If `src` is `None`, default to 0

        (absolute_path, source_line)
    }
}

impl AstVisitor for ContextLoader {
    fn visit_array_type_name(&mut self, node: &ArrayTypeName) -> Result<bool> {
        self.array_type_names
            .insert(node.clone(), self.last_source_unit_id);
        Ok(true)
    }

    fn visit_assignment(&mut self, node: &Assignment) -> Result<bool> {
        self.nodes
            .insert(node.id, ASTNode::Assignment(node.clone()));
        self.assignments
            .insert(node.clone(), self.last_source_unit_id);
        Ok(true)
    }

    fn visit_binary_operation(&mut self, node: &BinaryOperation) -> Result<bool> {
        self.nodes
            .insert(node.id, ASTNode::BinaryOperation(node.clone()));
        self.binary_operations
            .insert(node.clone(), self.last_source_unit_id);
        Ok(true)
    }

    fn visit_block(&mut self, node: &Block) -> Result<bool> {
        self.nodes.insert(node.id, ASTNode::Block(node.clone()));
        self.blocks.insert(node.clone(), self.last_source_unit_id);
        Ok(true)
    }

    fn visit_conditional(&mut self, node: &Conditional) -> Result<bool> {
        self.nodes
            .insert(node.id, ASTNode::Conditional(node.clone()));
        self.conditionals
            .insert(node.clone(), self.last_source_unit_id);
        Ok(true)
    }

    fn visit_contract_definition(&mut self, node: &ContractDefinition) -> Result<bool> {
        self.nodes
            .insert(node.id, ASTNode::ContractDefinition(node.clone()));
        self.contract_definitions
            .insert(node.clone(), self.last_source_unit_id);
        Ok(true)
    }

    fn visit_elementary_type_name(&mut self, node: &ElementaryTypeName) -> Result<bool> {
        self.elementary_type_names
            .insert(node.clone(), self.last_source_unit_id);
        Ok(true)
    }

    fn visit_elementary_type_name_expression(
        &mut self,
        node: &ElementaryTypeNameExpression,
    ) -> Result<bool> {
        self.nodes
            .insert(node.id, ASTNode::ElementaryTypeNameExpression(node.clone()));
        self.elementary_type_name_expressions
            .insert(node.clone(), self.last_source_unit_id);
        Ok(true)
    }

    fn visit_emit_statement(&mut self, node: &EmitStatement) -> Result<bool> {
        self.emit_statements
            .insert(node.clone(), self.last_source_unit_id);
        Ok(true)
    }

    fn visit_enum_definition(&mut self, node: &EnumDefinition) -> Result<bool> {
        self.nodes
            .insert(node.id, ASTNode::EnumDefinition(node.clone()));
        self.enum_definitions
            .insert(node.clone(), self.last_source_unit_id);
        Ok(true)
    }

    fn visit_enum_value(&mut self, node: &EnumValue) -> Result<bool> {
        self.nodes.insert(node.id, ASTNode::EnumValue(node.clone()));
        self.enum_values
            .insert(node.clone(), self.last_source_unit_id);
        Ok(true)
    }

    fn visit_event_definition(&mut self, node: &EventDefinition) -> Result<bool> {
        self.nodes
            .insert(node.id, ASTNode::EventDefinition(node.clone()));
        self.event_definitions
            .insert(node.clone(), self.last_source_unit_id);
        Ok(true)
    }

    fn visit_error_definition(&mut self, node: &ErrorDefinition) -> Result<bool> {
        self.nodes
            .insert(node.id, ASTNode::ErrorDefinition(node.clone()));
        self.error_definitions
            .insert(node.clone(), self.last_source_unit_id);
        Ok(true)
    }

    fn visit_expression_statement(&mut self, node: &ExpressionStatement) -> Result<bool> {
        self.expression_statements
            .insert(node.clone(), self.last_source_unit_id);
        Ok(true)
    }

    fn visit_function_call(&mut self, node: &FunctionCall) -> Result<bool> {
        self.nodes
            .insert(node.id, ASTNode::FunctionCall(node.clone()));
        self.function_calls
            .insert(node.clone(), self.last_source_unit_id);
        Ok(true)
    }

    fn visit_function_call_options(&mut self, node: &FunctionCallOptions) -> Result<bool> {
        self.nodes
            .insert(node.id, ASTNode::FunctionCallOptions(node.clone()));
        self.function_call_options
            .insert(node.clone(), self.last_source_unit_id);
        Ok(true)
    }

    fn visit_function_definition(&mut self, node: &FunctionDefinition) -> Result<bool> {
        self.nodes
            .insert(node.id, ASTNode::FunctionDefinition(node.clone()));
        self.function_definitions
            .insert(node.clone(), self.last_source_unit_id);
        Ok(true)
    }

    fn visit_function_type_name(&mut self, node: &FunctionTypeName) -> Result<bool> {
        self.function_type_names
            .insert(node.clone(), self.last_source_unit_id);
        Ok(true)
    }

    fn visit_for_statement(&mut self, node: &ForStatement) -> Result<bool> {
        self.nodes
            .insert(node.id, ASTNode::ForStatement(node.clone()));
        self.for_statements
            .insert(node.clone(), self.last_source_unit_id);
        Ok(true)
    }

    fn visit_identifier(&mut self, node: &Identifier) -> Result<bool> {
        self.nodes
            .insert(node.id, ASTNode::Identifier(node.clone()));
        self.identifiers
            .insert(node.clone(), self.last_source_unit_id);
        Ok(true)
    }

    fn visit_identifier_path(&mut self, node: &IdentifierPath) -> Result<bool> {
        self.nodes
            .insert(node.id, ASTNode::IdentifierPath(node.clone()));
        self.identifier_paths
            .insert(node.clone(), self.last_source_unit_id);
        Ok(true)
    }

    fn visit_if_statement(&mut self, node: &IfStatement) -> Result<bool> {
        self.nodes
            .insert(node.id, ASTNode::IfStatement(node.clone()));
        self.if_statements
            .insert(node.clone(), self.last_source_unit_id);
        Ok(true)
    }

    fn visit_import_directive(&mut self, node: &ImportDirective) -> Result<bool> {
        self.nodes
            .insert(node.id, ASTNode::ImportDirective(node.clone()));
        self.import_directives
            .insert(node.clone(), self.last_source_unit_id);
        Ok(true)
    }

    fn visit_index_access(&mut self, node: &IndexAccess) -> Result<bool> {
        self.nodes
            .insert(node.id, ASTNode::IndexAccess(node.clone()));
        self.index_accesses
            .insert(node.clone(), self.last_source_unit_id);
        Ok(true)
    }

    fn visit_index_range_access(&mut self, node: &IndexRangeAccess) -> Result<bool> {
        self.nodes
            .insert(node.id, ASTNode::IndexRangeAccess(node.clone()));
        self.index_range_accesses
            .insert(node.clone(), self.last_source_unit_id);
        Ok(true)
    }

    fn visit_inheritance_specifier(&mut self, node: &InheritanceSpecifier) -> Result<bool> {
        self.nodes
            .insert(node.id, ASTNode::InheritanceSpecifier(node.clone()));
        self.inheritance_specifiers
            .insert(node.clone(), self.last_source_unit_id);
        Ok(true)
    }

    fn visit_inline_assembly(&mut self, node: &InlineAssembly) -> Result<bool> {
        self.nodes
            .insert(node.id, ASTNode::InlineAssembly(node.clone()));
        self.inline_assemblies
            .insert(node.clone(), self.last_source_unit_id);
        Ok(true)
    }

    fn visit_literal(&mut self, node: &Literal) -> Result<bool> {
        self.nodes.insert(node.id, ASTNode::Literal(node.clone()));
        self.literals.insert(node.clone(), self.last_source_unit_id);
        Ok(true)
    }

    fn visit_member_access(&mut self, node: &MemberAccess) -> Result<bool> {
        self.nodes
            .insert(node.id, ASTNode::MemberAccess(node.clone()));
        self.member_accesses
            .insert(node.clone(), self.last_source_unit_id);
        Ok(true)
    }

    fn visit_new_expression(&mut self, node: &NewExpression) -> Result<bool> {
        self.nodes
            .insert(node.id, ASTNode::NewExpression(node.clone()));
        self.new_expressions
            .insert(node.clone(), self.last_source_unit_id);
        Ok(true)
    }

    fn visit_mapping(&mut self, node: &Mapping) -> Result<bool> {
        self.mappings.insert(node.clone(), self.last_source_unit_id);
        Ok(true)
    }

    fn visit_modifier_definition(&mut self, node: &ModifierDefinition) -> Result<bool> {
        self.nodes
            .insert(node.id, ASTNode::ModifierDefinition(node.clone()));
        self.modifier_definitions
            .insert(node.clone(), self.last_source_unit_id);
        Ok(true)
    }

    fn visit_modifier_invocation(&mut self, node: &ModifierInvocation) -> Result<bool> {
        self.nodes
            .insert(node.id, ASTNode::ModifierInvocation(node.clone()));
        self.modifier_invocations
            .insert(node.clone(), self.last_source_unit_id);
        Ok(true)
    }

    fn visit_override_specifier(&mut self, node: &OverrideSpecifier) -> Result<bool> {
        self.nodes
            .insert(node.id, ASTNode::OverrideSpecifier(node.clone()));
        self.override_specifiers
            .insert(node.clone(), self.last_source_unit_id);
        Ok(true)
    }

    fn visit_parameter_list(&mut self, node: &ParameterList) -> Result<bool> {
        self.nodes
            .insert(node.id, ASTNode::ParameterList(node.clone()));
        self.parameter_lists
            .insert(node.clone(), self.last_source_unit_id);
        Ok(true)
    }

    fn visit_pragma_directive(&mut self, node: &PragmaDirective) -> Result<bool> {
        self.nodes
            .insert(node.id, ASTNode::PragmaDirective(node.clone()));
        self.pragma_directives
            .insert(node.clone(), self.last_source_unit_id);
        Ok(true)
    }

    fn visit_return(&mut self, node: &Return) -> Result<bool> {
        self.nodes.insert(node.id, ASTNode::Return(node.clone()));
        self.returns.insert(node.clone(), self.last_source_unit_id);
        Ok(true)
    }

    fn visit_revert_statement(&mut self, node: &RevertStatement) -> Result<bool> {
        self.revert_statements
            .insert(node.clone(), self.last_source_unit_id);
        Ok(true)
    }

    fn visit_source_unit(&mut self, node: &SourceUnit) -> Result<bool> {
        self.nodes
            .insert(node.id, ASTNode::SourceUnit(node.clone()));
        self.source_units.push(node.clone());
        self.last_source_unit_id = node.id;
        Ok(true)
    }

    fn visit_struct_definition(&mut self, node: &StructDefinition) -> Result<bool> {
        self.nodes
            .insert(node.id, ASTNode::StructDefinition(node.clone()));
        self.struct_definitions
            .insert(node.clone(), self.last_source_unit_id);
        Ok(true)
    }

    fn visit_structured_documentation(&mut self, node: &StructuredDocumentation) -> Result<bool> {
        self.nodes
            .insert(node.id, ASTNode::StructuredDocumentation(node.clone()));
        self.structured_documentations
            .insert(node.clone(), self.last_source_unit_id);
        Ok(true)
    }

    fn visit_try_statement(&mut self, node: &TryStatement) -> Result<bool> {
        self.try_statements
            .insert(node.clone(), self.last_source_unit_id);
        Ok(true)
    }

    fn visit_try_catch_clause(&mut self, node: &TryCatchClause) -> Result<bool> {
        self.try_catch_clauses
            .insert(node.clone(), self.last_source_unit_id);
        Ok(true)
    }

    fn visit_tuple_expression(&mut self, node: &TupleExpression) -> Result<bool> {
        self.nodes
            .insert(node.id, ASTNode::TupleExpression(node.clone()));
        self.tuple_expressions
            .insert(node.clone(), self.last_source_unit_id);
        Ok(true)
    }

    fn visit_unary_operation(&mut self, node: &UnaryOperation) -> Result<bool> {
        self.nodes
            .insert(node.id, ASTNode::UnaryOperation(node.clone()));
        self.unary_operations
            .insert(node.clone(), self.last_source_unit_id);
        Ok(true)
    }

    fn visit_user_defined_type_name(&mut self, node: &UserDefinedTypeName) -> Result<bool> {
        self.user_defined_type_names
            .insert(node.clone(), self.last_source_unit_id);
        Ok(true)
    }

    fn visit_user_defined_value_type_definition(
        &mut self,
        node: &UserDefinedValueTypeDefinition,
    ) -> Result<bool> {
        self.nodes.insert(
            node.id,
            ASTNode::UserDefinedValueTypeDefinition(node.clone()),
        );
        self.user_defined_value_type_definitions
            .insert(node.clone(), self.last_source_unit_id);
        Ok(true)
    }

    fn visit_using_for_directive(&mut self, node: &UsingForDirective) -> Result<bool> {
        self.nodes
            .insert(node.id, ASTNode::UsingForDirective(node.clone()));
        self.using_for_directives
            .insert(node.clone(), self.last_source_unit_id);
        Ok(true)
    }

    fn visit_variable_declaration(&mut self, node: &VariableDeclaration) -> Result<bool> {
        self.nodes
            .insert(node.id, ASTNode::VariableDeclaration(node.clone()));
        self.variable_declarations
            .insert(node.clone(), self.last_source_unit_id);
        Ok(true)
    }

    fn visit_variable_declaration_statement(
        &mut self,
        node: &VariableDeclarationStatement,
    ) -> Result<bool> {
        self.nodes
            .insert(node.id, ASTNode::VariableDeclarationStatement(node.clone()));
        self.variable_declaration_statements
            .insert(node.clone(), self.last_source_unit_id);
        Ok(true)
    }

    fn visit_while_statement(&mut self, node: &WhileStatement) -> Result<bool> {
        self.nodes
            .insert(node.id, ASTNode::WhileStatement(node.clone()));
        self.while_statements
            .insert(node.clone(), self.last_source_unit_id);
        Ok(true)
    }
}

#[cfg(test)]
mod loader_tests {
    use crate::ast::*;
    use crate::context::loader::ContextLoader;
    use crate::framework::foundry::FoundryOutput;
    use crate::visitor::ast_visitor::*;
    use eyre::Result;

    fn read_compiler_output(filepath: &str) -> Result<FoundryOutput> {
        Ok(serde_json::from_reader(std::io::BufReader::new(
            std::fs::File::open(filepath)?,
        ))?)
    }

    #[derive(Default, Debug)]
    pub struct DelegateCallInLoopDetector {
        pub found_delegate_call_in_loop: Vec<MemberAccess>,
    }

    impl ASTConstVisitor for DelegateCallInLoopDetector {
        fn visit_member_access(&mut self, node: &MemberAccess) -> Result<bool> {
            if node.member_name == "delegatecall" {
                self.found_delegate_call_in_loop.push(node.clone());
            }
            Ok(true)
        }
    }

    #[test]
    fn test_delegate_call_in_loops() -> Result<()> {
        let mut loader = ContextLoader::default();
        let extended_inheritance = read_compiler_output(
            "tests/contract-playground/out/ExtendedInheritance.sol/ExtendedInheritance.json",
        )?;
        let inheritance_base = read_compiler_output(
            "tests/contract-playground/out/InheritanceBase.sol/InheritanceBase.json",
        )?;
        let i_contract_inheritance = read_compiler_output(
            "tests/contract-playground/out/IContractInheritance.sol/IContractInheritance.json",
        )?;
        extended_inheritance.ast.accept(&mut loader)?;
        inheritance_base.ast.accept(&mut loader)?;
        i_contract_inheritance.ast.accept(&mut loader)?;

        // Get all for statements, and check if there is a delegate call in the body of each for statement
        let mut delegate_call_in_loop_detector = DelegateCallInLoopDetector::default();
        let for_statements = loader.get_for_statements();
        for for_statement in for_statements {
            for_statement.accept(&mut delegate_call_in_loop_detector)?;
        }
        println!(
            "Found delegate call in loop: {:?}",
            delegate_call_in_loop_detector.found_delegate_call_in_loop
        );

        Ok(())
    }
}
