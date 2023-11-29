use eyre::Result;
use std::collections::HashMap;
use tokei::Language;

use crate::ast::*;

#[derive(Debug, PartialEq)]
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

// TODO: Instead of storing the ...Context items here, just go back to what we had before, and
// use the visitors to build a more comprehensive context object. This will allow us to only
// need lifetime parameters for the context object, and not the ASTNode enum.

#[derive(Default, Debug)]
pub struct ContextLoader<'a, 'b, 'c> {
    pub sloc_stats: Language,
    pub nodes: HashMap<i64, &'a mut ASTNode<'a, 'b, 'c>>,

    // Vectors of all context nodes, per type
    pub source_units: Vec<&'a mut SourceUnitContext<'a>>,
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
}

impl AstVisitor for ContextLoader<'_, '_, '_> {
    fn visit_source_unit<'a>(
        &mut self,
        context: &mut SourceUnitContext<'a>,
    ) -> std::io::Result<()> {
        self.source_units.push(context);
        self.nodes.insert(
            context.current_source_unit.id,
            ASTNode::SourceUnitContext(context.clone()),
        );
        Ok(())
    }
}

#[cfg(test)]
mod loader_tests {
    use crate::ast::*;
    use crate::context::loader::ContextLoader;
    use crate::framework::foundry::FoundryOutput;
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

    impl AstVisitor for DelegateCallInLoopDetector {
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
