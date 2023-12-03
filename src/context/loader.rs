use crate::ast::*;
use eyre::Result;
use std::collections::HashMap;
use tokei::Language;

#[derive(Debug, Clone, PartialEq)]
pub enum ASTNode {
    ArrayTypeName(ArrayTypeName),
    Assignment(Assignment),
    BinaryOperation(BinaryOperation),
    Block(Block),
    Conditional(Conditional),
    ContractDefinition(ContractDefinition),
    ElementaryTypeName(ElementaryTypeName),
    ElementaryTypeNameExpression(ElementaryTypeNameExpression),
    EmitStatement(EmitStatement),
    EnumDefinition(EnumDefinition),
    EnumValue(EnumValue),
    EventDefinition(EventDefinition),
    ErrorDefinition(ErrorDefinition),
    ExpressionStatement(ExpressionStatement),
    FunctionCall(FunctionCall),
    FunctionCallOptions(FunctionCallOptions),
    FunctionDefinition(FunctionDefinition),
    FunctionTypeName(FunctionTypeName),
    ForStatement(ForStatement),
    Identifier(Identifier),
    IdentifierPath(IdentifierPath),
    IfStatement(IfStatement),
    ImportDirective(ImportDirective),
    IndexAccess(IndexAccess),
    IndexRangeAccess(IndexRangeAccess),
    InheritanceSpecifier(InheritanceSpecifier),
    InlineAssembly(InlineAssembly),
    Literal(Literal),
    MemberAccess(MemberAccess),
    NewExpression(NewExpression),
    Mapping(Mapping),
    ModifierDefinition(ModifierDefinition),
    ModifierInvocation(ModifierInvocation),
    OverrideSpecifier(OverrideSpecifier),
    ParameterList(ParameterList),
    PragmaDirective(PragmaDirective),
    Return(Return),
    RevertStatement(RevertStatement),
    SourceUnit(SourceUnit),
    StructDefinition(StructDefinition),
    StructuredDocumentation(StructuredDocumentation),
    TryStatement(TryStatement),
    TryCatchClause(TryCatchClause),
    TupleExpression(TupleExpression),
    UnaryOperation(UnaryOperation),
    UserDefinedTypeName(UserDefinedTypeName),
    UserDefinedValueTypeDefinition(UserDefinedValueTypeDefinition),
    UsingForDirective(UsingForDirective),
    VariableDeclaration(VariableDeclaration),
    VariableDeclarationStatement(VariableDeclarationStatement),
    WhileStatement(WhileStatement),
    YulLiteral(YulLiteral),
    YulTypedName(YulTypedName),
    YulIf(YulIf),
    YulSwitch(YulSwitch),
    YulCase(YulCase),
    YulForLoop(YulForLoop),
    YulFunctionDefinition(YulFunctionDefinition),
    YulFunctionCall(YulFunctionCall),
    YulExpressionStatement(YulExpressionStatement),
    YulAssignment(YulAssignment),
    YulIdentifier(YulIdentifier),
    YulVariableDeclaration(YulVariableDeclaration),
    YulBlock(YulBlock),
}

impl ASTNode {
    pub fn src(&self) -> Option<&str> {
        match self {
            ASTNode::ArrayTypeName(_) => None,
            ASTNode::Assignment(node) => Some(&node.src),
            ASTNode::BinaryOperation(node) => Some(&node.src),
            ASTNode::Block(node) => Some(&node.src),
            ASTNode::Conditional(node) => Some(&node.src),
            ASTNode::ContractDefinition(node) => Some(&node.src),
            ASTNode::ElementaryTypeName(_) => None,
            ASTNode::ElementaryTypeNameExpression(node) => Some(&node.src),
            ASTNode::EmitStatement(_) => None,
            ASTNode::EnumDefinition(node) => Some(&node.src),
            ASTNode::EnumValue(node) => Some(&node.src),
            ASTNode::EventDefinition(node) => Some(&node.src),
            ASTNode::ErrorDefinition(node) => Some(&node.src),
            ASTNode::ExpressionStatement(_) => None,
            ASTNode::FunctionCall(node) => Some(&node.src),
            ASTNode::FunctionCallOptions(node) => Some(&node.src),
            ASTNode::FunctionDefinition(node) => Some(&node.src),
            ASTNode::FunctionTypeName(_) => None,
            ASTNode::ForStatement(node) => Some(&node.src),
            ASTNode::Identifier(node) => Some(&node.src),
            ASTNode::IdentifierPath(node) => Some(&node.src),
            ASTNode::IfStatement(node) => Some(&node.src),
            ASTNode::ImportDirective(node) => Some(&node.src),
            ASTNode::IndexAccess(node) => Some(&node.src),
            ASTNode::IndexRangeAccess(node) => Some(&node.src),
            ASTNode::InheritanceSpecifier(node) => Some(&node.src),
            ASTNode::InlineAssembly(node) => Some(&node.src),
            ASTNode::Literal(node) => Some(&node.src),
            ASTNode::MemberAccess(node) => Some(&node.src),
            ASTNode::NewExpression(node) => Some(&node.src),
            ASTNode::Mapping(_) => None,
            ASTNode::ModifierDefinition(node) => Some(&node.src),
            ASTNode::ModifierInvocation(node) => Some(&node.src),
            ASTNode::OverrideSpecifier(node) => Some(&node.src),
            ASTNode::ParameterList(node) => Some(&node.src),
            ASTNode::PragmaDirective(node) => Some(&node.src),
            ASTNode::Return(node) => Some(&node.src),
            ASTNode::RevertStatement(_) => None,
            ASTNode::SourceUnit(_) => None,
            ASTNode::StructDefinition(node) => Some(&node.src),
            ASTNode::StructuredDocumentation(node) => Some(&node.src),
            ASTNode::TryStatement(_) => None,
            ASTNode::TryCatchClause(_) => None,
            ASTNode::TupleExpression(node) => Some(&node.src),
            ASTNode::UnaryOperation(node) => Some(&node.src),
            ASTNode::UserDefinedTypeName(_) => None,
            ASTNode::UserDefinedValueTypeDefinition(node) => Some(&node.src),
            ASTNode::UsingForDirective(node) => Some(&node.src),
            ASTNode::VariableDeclaration(node) => Some(&node.src),
            ASTNode::VariableDeclarationStatement(node) => Some(&node.src),
            ASTNode::WhileStatement(node) => Some(&node.src),
            ASTNode::YulLiteral(_) => todo!(),
            ASTNode::YulTypedName(_) => todo!(),
            ASTNode::YulIf(_) => todo!(),
            ASTNode::YulSwitch(_) => todo!(),
            ASTNode::YulCase(_) => todo!(),
            ASTNode::YulForLoop(_) => todo!(),
            ASTNode::YulFunctionDefinition(_) => todo!(),
            ASTNode::YulFunctionCall(_) => todo!(),
            ASTNode::YulExpressionStatement(_) => todo!(),
            ASTNode::YulAssignment(_) => todo!(),
            ASTNode::YulIdentifier(_) => todo!(),
            ASTNode::YulVariableDeclaration(_) => todo!(),
            ASTNode::YulBlock(_) => todo!(),
        }
    }

    fn id(&self) -> i64 {
        match self {
            ASTNode::Assignment(node) => node.id,
            ASTNode::BinaryOperation(node) => node.id,
            ASTNode::Block(node) => node.id,
            ASTNode::Conditional(node) => node.id,
            ASTNode::ContractDefinition(node) => node.id,
            ASTNode::ElementaryTypeNameExpression(node) => node.id,
            ASTNode::EmitStatement(node) => match &node.event_call {
                Expression::FunctionCall(node) => node.id,
                Expression::Literal(node) => node.id,
                Expression::Identifier(node) => node.id,
                Expression::MemberAccess(node) => node.id,
                Expression::IndexAccess(node) => node.id,
                Expression::IndexRangeAccess(node) => node.id,
                Expression::UnaryOperation(node) => node.id,
                Expression::BinaryOperation(node) => node.id,
                Expression::Conditional(node) => node.id,
                Expression::TupleExpression(node) => node.id,
                Expression::Assignment(node) => node.id,
                Expression::FunctionCallOptions(node) => node.id,
                Expression::ElementaryTypeNameExpression(node) => node.id,
                Expression::NewExpression(node) => node.id,
            },
            ASTNode::EnumDefinition(node) => node.id,
            ASTNode::EnumValue(node) => node.id,
            ASTNode::EventDefinition(node) => node.id,
            ASTNode::ErrorDefinition(node) => node.id,
            ASTNode::FunctionCall(node) => node.id,
            ASTNode::FunctionCallOptions(node) => node.id,
            ASTNode::FunctionDefinition(node) => node.id,
            ASTNode::ForStatement(node) => node.id,
            ASTNode::Identifier(node) => node.id,
            ASTNode::IdentifierPath(node) => node.id,
            ASTNode::IfStatement(node) => node.id,
            ASTNode::ImportDirective(node) => node.id,
            ASTNode::IndexAccess(node) => node.id,
            ASTNode::IndexRangeAccess(node) => node.id,
            ASTNode::InheritanceSpecifier(node) => node.id,
            ASTNode::InlineAssembly(node) => node.id,
            ASTNode::Literal(node) => node.id,
            ASTNode::MemberAccess(node) => node.id,
            ASTNode::NewExpression(node) => node.id,
            ASTNode::ModifierDefinition(node) => node.id,
            ASTNode::ModifierInvocation(node) => node.id,
            ASTNode::OverrideSpecifier(node) => node.id,
            ASTNode::ParameterList(node) => node.id,
            ASTNode::PragmaDirective(node) => node.id,
            ASTNode::Return(node) => node.id,
            ASTNode::RevertStatement(node) => node.error_call.id,
            ASTNode::SourceUnit(node) => node.id,
            ASTNode::StructDefinition(node) => node.id,
            ASTNode::StructuredDocumentation(node) => node.id,
            ASTNode::TupleExpression(node) => node.id,
            ASTNode::UnaryOperation(node) => node.id,
            ASTNode::UserDefinedValueTypeDefinition(node) => node.id,
            ASTNode::UsingForDirective(node) => node.id,
            ASTNode::VariableDeclaration(node) => node.id,
            ASTNode::VariableDeclarationStatement(node) => node.id,
            ASTNode::WhileStatement(node) => node.id,
            ASTNode::YulLiteral(_) => todo!(),
            ASTNode::YulTypedName(_) => todo!(),
            ASTNode::YulIf(_) => todo!(),
            ASTNode::YulSwitch(_) => todo!(),
            ASTNode::YulCase(_) => todo!(),
            ASTNode::YulForLoop(_) => todo!(),
            ASTNode::YulFunctionDefinition(_) => todo!(),
            ASTNode::YulFunctionCall(_) => todo!(),
            ASTNode::YulExpressionStatement(_) => todo!(),
            ASTNode::YulAssignment(_) => todo!(),
            ASTNode::YulIdentifier(_) => todo!(),
            ASTNode::YulVariableDeclaration(_) => todo!(),
            ASTNode::YulBlock(_) => todo!(),
            ASTNode::ArrayTypeName(_) => todo!(),
            ASTNode::ElementaryTypeName(_) => todo!(),
            ASTNode::ExpressionStatement(_) => todo!(),
            ASTNode::FunctionTypeName(_) => todo!(),
            ASTNode::Mapping(_) => todo!(),
            ASTNode::TryStatement(_) => todo!(),
            ASTNode::TryCatchClause(_) => todo!(),
            ASTNode::UserDefinedTypeName(_) => todo!(),
        }
    }
}

#[derive(Default, Debug)]
pub struct ContextLoader {
    pub sloc_stats: Language,
    pub nodes: HashMap<i64, ASTNode>,
    pub node_id_to_source_unit_id: HashMap<i64, i64>,

    // Vectors of nodes
    pub source_units: Vec<SourceUnit>,
    pub assignments: Vec<Assignment>,
    pub binary_operations: Vec<BinaryOperation>,
    pub blocks: Vec<Block>,
    pub conditionals: Vec<Conditional>,
    pub contract_definitions: Vec<ContractDefinition>,
    pub elementary_type_name_expressions: Vec<ElementaryTypeNameExpression>,
    pub emit_statements: Vec<EmitStatement>,
    pub enum_definitions: Vec<EnumDefinition>,
    pub event_definitions: Vec<EventDefinition>,
    pub error_definitions: Vec<ErrorDefinition>,
    pub function_calls: Vec<FunctionCall>,
    pub function_call_options: Vec<FunctionCallOptions>,
    pub function_definitions: Vec<FunctionDefinition>,
    pub for_statements: Vec<ForStatement>,
    pub identifiers: Vec<Identifier>,
    pub if_statements: Vec<IfStatement>,
    pub import_directives: Vec<ImportDirective>,
    pub index_accesses: Vec<IndexAccess>,
    pub index_range_accesses: Vec<IndexRangeAccess>,
    pub inline_assemblies: Vec<InlineAssembly>,
    pub literals: Vec<Literal>,
    pub member_accesses: Vec<MemberAccess>,
    pub new_expressions: Vec<NewExpression>,
    pub modifier_definitions: Vec<ModifierDefinition>,
    pub modifier_invocations: Vec<ModifierInvocation>,
    pub pragma_directives: Vec<PragmaDirective>,
    pub returns: Vec<Return>,
    pub revert_statements: Vec<RevertStatement>,
    pub struct_definitions: Vec<StructDefinition>,
    pub try_statements: Vec<TryStatement>,
    pub tuple_expressions: Vec<TupleExpression>,
    pub unary_operations: Vec<UnaryOperation>,
    pub user_defined_value_type_definitions: Vec<UserDefinedValueTypeDefinition>,
    pub using_for_directives: Vec<UsingForDirective>,
    pub variable_declarations: Vec<VariableDeclaration>,
    pub variable_declaration_statements: Vec<VariableDeclarationStatement>,
    pub while_statements: Vec<WhileStatement>,
}

impl ContextLoader {
    // SETTERS

    pub fn set_sloc_stats(&mut self, sloc_stats: Language) {
        self.sloc_stats = sloc_stats;
    }

    // GETTERS

    pub fn get_source_unit_from_child_node(&self, node: &ASTNode) -> Option<&SourceUnit> {
        let source_unit_id = self.node_id_to_source_unit_id.get(&node.id())?;
        self.nodes.get(source_unit_id).and_then(|node| match node {
            ASTNode::SourceUnit(source_unit) => Some(source_unit),
            _ => None,
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

impl AstContextVisitor for ContextLoader {
    fn visit_source_unit<'a>(
        &mut self,
        context: &mut SourceUnitContext<'a>,
    ) -> std::io::Result<()> {
        self.source_units.push(context.current_source_unit.clone());
        self.nodes.insert(
            context.current_source_unit.id,
            ASTNode::SourceUnit(context.current_source_unit.clone()),
        );
        Ok(())
    }

    fn visit_assignment<'a, 'b>(
        &mut self,
        context: &mut AssignmentContext<'a, 'b>,
    ) -> std::io::Result<()> {
        self.assignments.push(context.assignment.clone());
        self.nodes.insert(
            context.assignment.id,
            ASTNode::Assignment(context.assignment.clone()),
        );
        self.node_id_to_source_unit_id
            .insert(context.assignment.id, context.current_source_unit.id);
        Ok(())
    }

    fn visit_binary_operation<'a, 'b>(
        &mut self,
        context: &mut BinaryOperationContext<'a, 'b>,
    ) -> std::io::Result<()> {
        self.binary_operations
            .push(context.binary_operation.clone());
        self.nodes.insert(
            context.binary_operation.id,
            ASTNode::BinaryOperation(context.binary_operation.clone()),
        );
        self.node_id_to_source_unit_id
            .insert(context.binary_operation.id, context.current_source_unit.id);
        Ok(())
    }

    fn visit_block<'a, 'b>(&mut self, context: &mut BlockContext<'a, 'b>) -> std::io::Result<()> {
        self.blocks.push(context.block.clone());
        self.nodes
            .insert(context.block.id, ASTNode::Block(context.block.clone()));
        self.node_id_to_source_unit_id
            .insert(context.block.id, context.current_source_unit.id);
        Ok(())
    }

    fn visit_conditional<'a, 'b>(
        &mut self,
        context: &mut ConditionalContext<'a, 'b>,
    ) -> std::io::Result<()> {
        self.conditionals.push(context.conditional.clone());
        self.nodes.insert(
            context.conditional.id,
            ASTNode::Conditional(context.conditional.clone()),
        );
        self.node_id_to_source_unit_id
            .insert(context.conditional.id, context.current_source_unit.id);
        Ok(())
    }

    fn visit_contract_definition<'a, 'b>(
        &mut self,
        context: &mut ContractDefinitionContext<'a>,
    ) -> std::io::Result<()> {
        self.contract_definitions
            .push(context.contract_definition.clone());
        self.nodes.insert(
            context.contract_definition.id,
            ASTNode::ContractDefinition(context.contract_definition.clone()),
        );
        self.node_id_to_source_unit_id.insert(
            context.contract_definition.id,
            context.current_source_unit.id,
        );
        Ok(())
    }

    fn visit_elementary_type_name_expression<'a, 'b>(
        &mut self,
        context: &mut ElementaryTypeNameExpressionContext<'a, 'b>,
    ) -> std::io::Result<()> {
        self.elementary_type_name_expressions
            .push(context.elementary_type_name_expression.clone());
        self.nodes.insert(
            context.elementary_type_name_expression.id,
            ASTNode::ElementaryTypeNameExpression(context.elementary_type_name_expression.clone()),
        );
        self.node_id_to_source_unit_id.insert(
            context.elementary_type_name_expression.id,
            context.current_source_unit.id,
        );
        Ok(())
    }

    fn visit_emit_statement<'a, 'b>(
        &mut self,
        context: &mut EmitStatementContext<'a, 'b>,
    ) -> std::io::Result<()> {
        self.emit_statements.push(context.emit_statement.clone());
        // Match context.emit_statement.event_call to any expression and get the ID from it
        let event_call_id = match &context.emit_statement.event_call {
            Expression::FunctionCall(node) => node.id,
            Expression::Literal(node) => node.id,
            Expression::Identifier(node) => node.id,
            Expression::MemberAccess(node) => node.id,
            Expression::IndexAccess(node) => node.id,
            Expression::IndexRangeAccess(node) => node.id,
            Expression::UnaryOperation(node) => node.id,
            Expression::BinaryOperation(node) => node.id,
            Expression::Conditional(node) => node.id,
            Expression::TupleExpression(node) => node.id,
            Expression::Assignment(node) => node.id,
            Expression::FunctionCallOptions(node) => node.id,
            Expression::ElementaryTypeNameExpression(node) => node.id,
            Expression::NewExpression(node) => node.id,
        };
        self.nodes.insert(
            event_call_id,
            ASTNode::EmitStatement(context.emit_statement.clone()),
        );
        self.node_id_to_source_unit_id
            .insert(event_call_id, context.current_source_unit.id);
        Ok(())
    }

    fn visit_enum_definition<'a>(
        &mut self,
        context: &mut EnumDefinitionContext<'a>,
    ) -> std::io::Result<()> {
        self.enum_definitions.push(context.enum_definition.clone());
        self.nodes.insert(
            context.enum_definition.id,
            ASTNode::EnumDefinition(context.enum_definition.clone()),
        );
        self.node_id_to_source_unit_id
            .insert(context.enum_definition.id, context.current_source_unit.id);
        Ok(())
    }

    fn visit_event_definition<'a>(
        &mut self,
        context: &mut EventDefinitionContext<'a>,
    ) -> std::io::Result<()> {
        self.event_definitions
            .push(context.event_definition.clone());
        self.nodes.insert(
            context.event_definition.id,
            ASTNode::EventDefinition(context.event_definition.clone()),
        );
        self.node_id_to_source_unit_id
            .insert(context.event_definition.id, context.current_source_unit.id);
        Ok(())
    }

    fn visit_error_definition<'a>(
        &mut self,
        context: &mut ErrorDefinitionContext<'a>,
    ) -> std::io::Result<()> {
        self.error_definitions
            .push(context.error_definition.clone());
        self.nodes.insert(
            context.error_definition.id,
            ASTNode::ErrorDefinition(context.error_definition.clone()),
        );
        self.node_id_to_source_unit_id
            .insert(context.error_definition.id, context.current_source_unit.id);
        Ok(())
    }

    fn visit_function_call<'a, 'b>(
        &mut self,
        context: &mut FunctionCallContext<'a, 'b>,
    ) -> std::io::Result<()> {
        self.function_calls.push(context.function_call.clone());
        self.nodes.insert(
            context.function_call.id,
            ASTNode::FunctionCall(context.function_call.clone()),
        );
        self.node_id_to_source_unit_id
            .insert(context.function_call.id, context.current_source_unit.id);
        Ok(())
    }

    fn visit_function_call_options<'a, 'b>(
        &mut self,
        context: &mut FunctionCallOptionsContext<'a, 'b>,
    ) -> std::io::Result<()> {
        self.function_call_options
            .push(context.function_call_options.clone());
        self.nodes.insert(
            context.function_call_options.id,
            ASTNode::FunctionCallOptions(context.function_call_options.clone()),
        );
        self.node_id_to_source_unit_id.insert(
            context.function_call_options.id,
            context.current_source_unit.id,
        );
        Ok(())
    }

    fn visit_function_definition<'a>(
        &mut self,
        context: &mut FunctionDefinitionContext<'a>,
    ) -> std::io::Result<()> {
        self.function_definitions
            .push(context.function_definition.clone());
        self.nodes.insert(
            context.function_definition.id,
            ASTNode::FunctionDefinition(context.function_definition.clone()),
        );
        self.node_id_to_source_unit_id.insert(
            context.function_definition.id,
            context.current_source_unit.id,
        );
        Ok(())
    }

    fn visit_for_statement<'a, 'b>(
        &mut self,
        context: &mut ForStatementContext<'a, 'b>,
    ) -> std::io::Result<()> {
        self.for_statements.push(context.for_statement.clone());
        self.nodes.insert(
            context.for_statement.id,
            ASTNode::ForStatement(context.for_statement.clone()),
        );
        self.node_id_to_source_unit_id
            .insert(context.for_statement.id, context.current_source_unit.id);
        Ok(())
    }

    fn visit_identifier<'a, 'b>(
        &mut self,
        context: &mut IdentifierContext<'a, 'b>,
    ) -> std::io::Result<()> {
        self.identifiers.push(context.identifier.clone());
        self.nodes.insert(
            context.identifier.id,
            ASTNode::Identifier(context.identifier.clone()),
        );
        self.node_id_to_source_unit_id
            .insert(context.identifier.id, context.current_source_unit.id);
        Ok(())
    }

    fn visit_if_statement<'a, 'b>(
        &mut self,
        context: &mut IfStatementContext<'a, 'b>,
    ) -> std::io::Result<()> {
        self.if_statements.push(context.if_statement.clone());
        self.nodes.insert(
            context.if_statement.id,
            ASTNode::IfStatement(context.if_statement.clone()),
        );
        self.node_id_to_source_unit_id
            .insert(context.if_statement.id, context.current_source_unit.id);
        Ok(())
    }

    fn visit_import_directive<'a>(
        &mut self,
        context: &mut ImportDirectiveContext<'a>,
    ) -> std::io::Result<()> {
        self.import_directives
            .push(context.import_directive.clone());
        self.nodes.insert(
            context.import_directive.id,
            ASTNode::ImportDirective(context.import_directive.clone()),
        );
        self.node_id_to_source_unit_id
            .insert(context.import_directive.id, context.current_source_unit.id);
        Ok(())
    }

    fn visit_index_access<'a, 'b>(
        &mut self,
        context: &mut IndexAccessContext<'a, 'b>,
    ) -> std::io::Result<()> {
        self.index_accesses.push(context.index_access.clone());
        self.nodes.insert(
            context.index_access.id,
            ASTNode::IndexAccess(context.index_access.clone()),
        );
        self.node_id_to_source_unit_id
            .insert(context.index_access.id, context.current_source_unit.id);
        Ok(())
    }

    fn visit_index_range_access<'a, 'b>(
        &mut self,
        context: &mut IndexRangeAccessContext<'a, 'b>,
    ) -> std::io::Result<()> {
        self.index_range_accesses
            .push(context.index_range_access.clone());
        self.nodes.insert(
            context.index_range_access.id,
            ASTNode::IndexRangeAccess(context.index_range_access.clone()),
        );
        self.node_id_to_source_unit_id.insert(
            context.index_range_access.id,
            context.current_source_unit.id,
        );
        Ok(())
    }

    fn visit_inline_assembly<'a, 'b>(
        &mut self,
        context: &mut InlineAssemblyContext<'a, 'b>,
    ) -> std::io::Result<()> {
        self.inline_assemblies.push(context.inline_assembly.clone());
        self.nodes.insert(
            context.inline_assembly.id,
            ASTNode::InlineAssembly(context.inline_assembly.clone()),
        );
        self.node_id_to_source_unit_id
            .insert(context.inline_assembly.id, context.current_source_unit.id);
        Ok(())
    }

    fn visit_literal<'a, 'b>(
        &mut self,
        context: &mut LiteralContext<'a, 'b>,
    ) -> std::io::Result<()> {
        self.literals.push(context.literal.clone());
        self.nodes.insert(
            context.literal.id,
            ASTNode::Literal(context.literal.clone()),
        );
        self.node_id_to_source_unit_id
            .insert(context.literal.id, context.current_source_unit.id);
        Ok(())
    }

    fn visit_member_access<'a, 'b>(
        &mut self,
        context: &mut MemberAccessContext<'a, 'b>,
    ) -> std::io::Result<()> {
        self.member_accesses.push(context.member_access.clone());
        self.nodes.insert(
            context.member_access.id,
            ASTNode::MemberAccess(context.member_access.clone()),
        );
        self.node_id_to_source_unit_id
            .insert(context.member_access.id, context.current_source_unit.id);
        Ok(())
    }

    fn visit_new_expression<'a, 'b>(
        &mut self,
        context: &mut NewExpressionContext<'a, 'b>,
    ) -> std::io::Result<()> {
        self.new_expressions.push(context.new_expression.clone());
        self.nodes.insert(
            context.new_expression.id,
            ASTNode::NewExpression(context.new_expression.clone()),
        );
        self.node_id_to_source_unit_id
            .insert(context.new_expression.id, context.current_source_unit.id);
        Ok(())
    }

    fn visit_modifier_definition<'a>(
        &mut self,
        context: &mut ModifierDefinitionContext<'a>,
    ) -> std::io::Result<()> {
        self.modifier_definitions
            .push(context.modifier_definition.clone());
        self.nodes.insert(
            context.modifier_definition.id,
            ASTNode::ModifierDefinition(context.modifier_definition.clone()),
        );
        self.node_id_to_source_unit_id.insert(
            context.modifier_definition.id,
            context.current_source_unit.id,
        );
        Ok(())
    }

    fn visit_modifier_invocation<'a>(
        &mut self,
        context: &mut ModifierInvocationContext<'a>,
    ) -> std::io::Result<()> {
        self.modifier_invocations
            .push(context.modifier_invocation.clone());
        self.nodes.insert(
            context.modifier_invocation.id,
            ASTNode::ModifierInvocation(context.modifier_invocation.clone()),
        );
        self.node_id_to_source_unit_id.insert(
            context.modifier_invocation.id,
            context.current_source_unit.id,
        );
        Ok(())
    }

    fn visit_pragma_directive<'a>(
        &mut self,
        context: &mut PragmaDirectiveContext<'a>,
    ) -> std::io::Result<()> {
        self.pragma_directives
            .push(context.pragma_directive.clone());
        self.nodes.insert(
            context.pragma_directive.id,
            ASTNode::PragmaDirective(context.pragma_directive.clone()),
        );
        self.node_id_to_source_unit_id
            .insert(context.pragma_directive.id, context.current_source_unit.id);
        Ok(())
    }

    fn visit_return<'a, 'b>(&mut self, context: &mut ReturnContext<'a, 'b>) -> std::io::Result<()> {
        self.returns.push(context.return_statement.clone());
        self.nodes.insert(
            context.return_statement.id,
            ASTNode::Return(context.return_statement.clone()),
        );
        self.node_id_to_source_unit_id
            .insert(context.return_statement.id, context.current_source_unit.id);
        Ok(())
    }

    fn visit_revert_statement<'a, 'b>(
        &mut self,
        context: &mut RevertStatementContext<'a, 'b>,
    ) -> std::io::Result<()> {
        self.revert_statements
            .push(context.revert_statement.clone());
        self.nodes.insert(
            context.revert_statement.error_call.id,
            ASTNode::RevertStatement(context.revert_statement.clone()),
        );
        self.node_id_to_source_unit_id.insert(
            context.revert_statement.error_call.id,
            context.current_source_unit.id,
        );
        Ok(())
    }

    fn visit_struct_definition<'a>(
        &mut self,
        context: &mut StructDefinitionContext<'a>,
    ) -> std::io::Result<()> {
        self.struct_definitions
            .push(context.struct_definition.clone());
        self.nodes.insert(
            context.struct_definition.id,
            ASTNode::StructDefinition(context.struct_definition.clone()),
        );
        self.node_id_to_source_unit_id
            .insert(context.struct_definition.id, context.current_source_unit.id);
        Ok(())
    }

    fn visit_try_statement<'a, 'b>(
        &mut self,
        context: &mut TryStatementContext<'a, 'b>,
    ) -> std::io::Result<()> {
        self.try_statements.push(context.try_statement.clone());
        self.nodes.insert(
            context.try_statement.external_call.id,
            ASTNode::TryStatement(context.try_statement.clone()),
        );
        self.node_id_to_source_unit_id.insert(
            context.try_statement.external_call.id,
            context.current_source_unit.id,
        );
        Ok(())
    }

    fn visit_tuple_expression<'a, 'b>(
        &mut self,
        context: &mut TupleExpressionContext<'a, 'b>,
    ) -> std::io::Result<()> {
        self.tuple_expressions
            .push(context.tuple_expression.clone());
        self.nodes.insert(
            context.tuple_expression.id,
            ASTNode::TupleExpression(context.tuple_expression.clone()),
        );
        self.node_id_to_source_unit_id
            .insert(context.tuple_expression.id, context.current_source_unit.id);
        Ok(())
    }

    fn visit_unary_operation<'a, 'b>(
        &mut self,
        context: &mut UnaryOperationContext<'a, 'b>,
    ) -> std::io::Result<()> {
        self.unary_operations.push(context.unary_operation.clone());
        self.nodes.insert(
            context.unary_operation.id,
            ASTNode::UnaryOperation(context.unary_operation.clone()),
        );
        self.node_id_to_source_unit_id
            .insert(context.unary_operation.id, context.current_source_unit.id);
        Ok(())
    }

    fn visit_user_defined_value_type_definition<'a>(
        &mut self,
        context: &mut UserDefinedValueTypeDefinitionContext<'a>,
    ) -> std::io::Result<()> {
        self.user_defined_value_type_definitions
            .push(context.user_defined_value_type_definition.clone());
        self.nodes.insert(
            context.user_defined_value_type_definition.id,
            ASTNode::UserDefinedValueTypeDefinition(
                context.user_defined_value_type_definition.clone(),
            ),
        );
        self.node_id_to_source_unit_id.insert(
            context.user_defined_value_type_definition.id,
            context.current_source_unit.id,
        );
        Ok(())
    }

    fn visit_using_for_directive<'a>(
        &mut self,
        context: &mut UsingForDirectiveContext<'a>,
    ) -> std::io::Result<()> {
        self.using_for_directives
            .push(context.using_for_directive.clone());
        self.nodes.insert(
            context.using_for_directive.id,
            ASTNode::UsingForDirective(context.using_for_directive.clone()),
        );
        self.node_id_to_source_unit_id.insert(
            context.using_for_directive.id,
            context.current_source_unit.id,
        );
        Ok(())
    }

    fn visit_variable_declaration<'a, 'b>(
        &mut self,
        context: &mut VariableDeclarationContext<'a, 'b>,
    ) -> std::io::Result<()> {
        self.variable_declarations
            .push(context.variable_declaration.clone());
        self.nodes.insert(
            context.variable_declaration.id,
            ASTNode::VariableDeclaration(context.variable_declaration.clone()),
        );
        self.node_id_to_source_unit_id.insert(
            context.variable_declaration.id,
            context.current_source_unit.id,
        );
        Ok(())
    }

    fn visit_variable_declaration_statement<'a, 'b>(
        &mut self,
        context: &mut VariableDeclarationStatementContext<'a, 'b>,
    ) -> std::io::Result<()> {
        self.variable_declaration_statements
            .push(context.variable_declaration_statement.clone());
        self.nodes.insert(
            context.variable_declaration_statement.id,
            ASTNode::VariableDeclarationStatement(context.variable_declaration_statement.clone()),
        );
        self.node_id_to_source_unit_id.insert(
            context.variable_declaration_statement.id,
            context.current_source_unit.id,
        );
        Ok(())
    }

    fn visit_while_statement<'a, 'b>(
        &mut self,
        context: &mut WhileStatementContext<'a, 'b>,
    ) -> std::io::Result<()> {
        self.while_statements.push(context.while_statement.clone());
        self.nodes.insert(
            context.while_statement.id,
            ASTNode::WhileStatement(context.while_statement.clone()),
        );
        self.node_id_to_source_unit_id
            .insert(context.while_statement.id, context.current_source_unit.id);
        Ok(())
    }
}

// #[cfg(test)]
// mod loader_tests {
//     use crate::ast::*;
//     use crate::context::loader::ContextLoader;
//     use crate::framework::foundry::FoundryOutput;
//     use eyre::Result;

//     fn read_compiler_output(filepath: &str) -> Result<FoundryOutput> {
//         Ok(serde_json::from_reader(std::io::BufReader::new(
//             std::fs::File::open(filepath)?,
//         ))?)
//     }

//     #[derive(Default, Debug)]
//     pub struct DelegateCallInLoopDetector {
//         pub found_delegate_call_in_loop: Vec<MemberAccess>,
//     }

//     impl AstContextVisitor for DelegateCallInLoopDetector {
//         fn visit_member_access<'a, 'b>(
//             &mut self,
//             context: &mut MemberAccessContext<'a, 'b>,
//         ) -> std::io::Result<()> {
//             if context.member_access.member_name == "delegatecall" {
//                 self.found_delegate_call_in_loop
//                     .push(context.member_access.clone());
//             }
//             Ok(())
//         }
//     }

//     #[test]
//     fn test_delegate_call_in_loops() -> Result<()> {
//         let mut loader = ContextLoader::default();
//         let extended_inheritance = read_compiler_output(
//             "tests/contract-playground/out/ExtendedInheritance.sol/ExtendedInheritance.json",
//         )?;
//         let inheritance_base = read_compiler_output(
//             "tests/contract-playground/out/InheritanceBase.sol/InheritanceBase.json",
//         )?;
//         let i_contract_inheritance = read_compiler_output(
//             "tests/contract-playground/out/IContractInheritance.sol/IContractInheritance.json",
//         )?;
//         extended_inheritance.ast.accept(&mut loader)?;
//         inheritance_base.ast.accept(&mut loader)?;
//         i_contract_inheritance.ast.accept(&mut loader)?;

//         // Get all for statements, and check if there is a delegate call in the body of each for statement
//         let mut delegate_call_in_loop_detector = DelegateCallInLoopDetector::default();
//         let for_statements = loader.get_for_statements();
//         for for_statement in for_statements {
//             for_statement.accept(&mut delegate_call_in_loop_detector)?;
//         }
//         println!(
//             "Found delegate call in loop: {:?}",
//             delegate_call_in_loop_detector.found_delegate_call_in_loop
//         );

//         Ok(())
//     }
// }
