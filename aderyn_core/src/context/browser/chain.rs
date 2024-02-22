use std::collections::HashMap;

use crate::{
    ast::*,
    context::workspace_context::{ASTNode, WorkspaceContext},
};

/// GetParentChain allows us to grab the ancestral hirearchy of a given node in the AST
/// all the way upto the ContractDefinition
pub trait GetParentChain {
    /// Get the parent Chain of an ASTNode
    fn of(&self, context: &WorkspaceContext) -> Vec<ASTNode>;
}

impl GetParentChain for Assignment {
    fn of(&self, context: &WorkspaceContext) -> Vec<ASTNode> {
        get_parent_chain_of_child(self.id, context)
    }
}
impl GetParentChain for BinaryOperation {
    fn of(&self, context: &WorkspaceContext) -> Vec<ASTNode> {
        get_parent_chain_of_child(self.id, context)
    }
}
impl GetParentChain for Block {
    fn of(&self, context: &WorkspaceContext) -> Vec<ASTNode> {
        get_parent_chain_of_child(self.id, context)
    }
}
impl GetParentChain for Conditional {
    fn of(&self, context: &WorkspaceContext) -> Vec<ASTNode> {
        get_parent_chain_of_child(self.id, context)
    }
}
impl GetParentChain for ContractDefinition {
    fn of(&self, context: &WorkspaceContext) -> Vec<ASTNode> {
        get_parent_chain_of_child(self.id, context)
    }
}
impl GetParentChain for ElementaryTypeNameExpression {
    fn of(&self, context: &WorkspaceContext) -> Vec<ASTNode> {
        get_parent_chain_of_child(self.id, context)
    }
}
impl GetParentChain for EnumDefinition {
    fn of(&self, context: &WorkspaceContext) -> Vec<ASTNode> {
        get_parent_chain_of_child(self.id, context)
    }
}
impl GetParentChain for EnumValue {
    fn of(&self, context: &WorkspaceContext) -> Vec<ASTNode> {
        get_parent_chain_of_child(self.id, context)
    }
}
impl GetParentChain for EventDefinition {
    fn of(&self, context: &WorkspaceContext) -> Vec<ASTNode> {
        get_parent_chain_of_child(self.id, context)
    }
}
impl GetParentChain for ErrorDefinition {
    fn of(&self, context: &WorkspaceContext) -> Vec<ASTNode> {
        get_parent_chain_of_child(self.id, context)
    }
}
impl GetParentChain for FunctionCall {
    fn of(&self, context: &WorkspaceContext) -> Vec<ASTNode> {
        get_parent_chain_of_child(self.id, context)
    }
}
impl GetParentChain for FunctionCallOptions {
    fn of(&self, context: &WorkspaceContext) -> Vec<ASTNode> {
        get_parent_chain_of_child(self.id, context)
    }
}
impl GetParentChain for FunctionDefinition {
    fn of(&self, context: &WorkspaceContext) -> Vec<ASTNode> {
        get_parent_chain_of_child(self.id, context)
    }
}
impl GetParentChain for ForStatement {
    fn of(&self, context: &WorkspaceContext) -> Vec<ASTNode> {
        get_parent_chain_of_child(self.id, context)
    }
}
impl GetParentChain for Identifier {
    fn of(&self, context: &WorkspaceContext) -> Vec<ASTNode> {
        get_parent_chain_of_child(self.id, context)
    }
}
impl GetParentChain for IdentifierPath {
    fn of(&self, context: &WorkspaceContext) -> Vec<ASTNode> {
        get_parent_chain_of_child(self.id, context)
    }
}
impl GetParentChain for IfStatement {
    fn of(&self, context: &WorkspaceContext) -> Vec<ASTNode> {
        get_parent_chain_of_child(self.id, context)
    }
}
impl GetParentChain for ImportDirective {
    fn of(&self, context: &WorkspaceContext) -> Vec<ASTNode> {
        get_parent_chain_of_child(self.id, context)
    }
}
impl GetParentChain for IndexAccess {
    fn of(&self, context: &WorkspaceContext) -> Vec<ASTNode> {
        get_parent_chain_of_child(self.id, context)
    }
}
impl GetParentChain for IndexRangeAccess {
    fn of(&self, context: &WorkspaceContext) -> Vec<ASTNode> {
        get_parent_chain_of_child(self.id, context)
    }
}
impl GetParentChain for InheritanceSpecifier {
    fn of(&self, context: &WorkspaceContext) -> Vec<ASTNode> {
        get_parent_chain_of_child(self.id, context)
    }
}
impl GetParentChain for InlineAssembly {
    fn of(&self, context: &WorkspaceContext) -> Vec<ASTNode> {
        get_parent_chain_of_child(self.id, context)
    }
}
impl GetParentChain for Literal {
    fn of(&self, context: &WorkspaceContext) -> Vec<ASTNode> {
        get_parent_chain_of_child(self.id, context)
    }
}
impl GetParentChain for MemberAccess {
    fn of(&self, context: &WorkspaceContext) -> Vec<ASTNode> {
        get_parent_chain_of_child(self.id, context)
    }
}
impl GetParentChain for NewExpression {
    fn of(&self, context: &WorkspaceContext) -> Vec<ASTNode> {
        get_parent_chain_of_child(self.id, context)
    }
}
impl GetParentChain for ModifierDefinition {
    fn of(&self, context: &WorkspaceContext) -> Vec<ASTNode> {
        get_parent_chain_of_child(self.id, context)
    }
}
impl GetParentChain for ModifierInvocation {
    fn of(&self, context: &WorkspaceContext) -> Vec<ASTNode> {
        get_parent_chain_of_child(self.id, context)
    }
}
impl GetParentChain for OverrideSpecifier {
    fn of(&self, context: &WorkspaceContext) -> Vec<ASTNode> {
        get_parent_chain_of_child(self.id, context)
    }
}
impl GetParentChain for ParameterList {
    fn of(&self, context: &WorkspaceContext) -> Vec<ASTNode> {
        get_parent_chain_of_child(self.id, context)
    }
}
impl GetParentChain for PragmaDirective {
    fn of(&self, context: &WorkspaceContext) -> Vec<ASTNode> {
        get_parent_chain_of_child(self.id, context)
    }
}
impl GetParentChain for Return {
    fn of(&self, context: &WorkspaceContext) -> Vec<ASTNode> {
        get_parent_chain_of_child(self.id, context)
    }
}
impl GetParentChain for SourceUnit {
    fn of(&self, context: &WorkspaceContext) -> Vec<ASTNode> {
        get_parent_chain_of_child(self.id, context)
    }
}
impl GetParentChain for StructDefinition {
    fn of(&self, context: &WorkspaceContext) -> Vec<ASTNode> {
        get_parent_chain_of_child(self.id, context)
    }
}
impl GetParentChain for StructuredDocumentation {
    fn of(&self, context: &WorkspaceContext) -> Vec<ASTNode> {
        get_parent_chain_of_child(self.id, context)
    }
}
impl GetParentChain for TupleExpression {
    fn of(&self, context: &WorkspaceContext) -> Vec<ASTNode> {
        get_parent_chain_of_child(self.id, context)
    }
}
impl GetParentChain for UnaryOperation {
    fn of(&self, context: &WorkspaceContext) -> Vec<ASTNode> {
        get_parent_chain_of_child(self.id, context)
    }
}
impl GetParentChain for UserDefinedValueTypeDefinition {
    fn of(&self, context: &WorkspaceContext) -> Vec<ASTNode> {
        get_parent_chain_of_child(self.id, context)
    }
}
impl GetParentChain for UsingForDirective {
    fn of(&self, context: &WorkspaceContext) -> Vec<ASTNode> {
        get_parent_chain_of_child(self.id, context)
    }
}
impl GetParentChain for VariableDeclaration {
    fn of(&self, context: &WorkspaceContext) -> Vec<ASTNode> {
        get_parent_chain_of_child(self.id, context)
    }
}
impl GetParentChain for VariableDeclarationStatement {
    fn of(&self, context: &WorkspaceContext) -> Vec<ASTNode> {
        get_parent_chain_of_child(self.id, context)
    }
}
impl GetParentChain for WhileStatement {
    fn of(&self, context: &WorkspaceContext) -> Vec<ASTNode> {
        get_parent_chain_of_child(self.id, context)
    }
}

pub fn get_parent_chain_of_child(child_id: NodeID, context: &WorkspaceContext) -> Vec<ASTNode> {
    let node_ancestors = context.all_ancestors.get(&child_id).unwrap();
    node_ancestors.ancestors.clone()
}

/// This is called in preprocess and the output is stored in WorkspaceContext
/// Please do not re-call this function
pub(crate) fn get_all_ast_nodes(context: &WorkspaceContext) -> Vec<ASTNode> {
    let mut all_parents: Vec<ASTNode> = vec![];

    let array_type_names = context.array_type_names.keys().cloned().collect::<Vec<_>>();

    let assignments = context.assignments.keys().cloned().collect::<Vec<_>>();

    let binary_operations = context
        .binary_operations
        .keys()
        .cloned()
        .collect::<Vec<_>>();

    let blocks = context.blocks.keys().cloned().collect::<Vec<_>>();

    let conditionals = context.conditionals.keys().cloned().collect::<Vec<_>>();

    let contract_definitions = context
        .contract_definitions
        .keys()
        .cloned()
        .collect::<Vec<_>>();

    let elementary_type_names = context
        .elementary_type_names
        .keys()
        .cloned()
        .collect::<Vec<_>>();

    let elementary_type_name_expressions = context
        .elementary_type_name_expressions
        .keys()
        .cloned()
        .collect::<Vec<_>>();

    let emit_statements = context.emit_statements.keys().cloned().collect::<Vec<_>>();

    let enum_definitions = context.enum_definitions.keys().cloned().collect::<Vec<_>>();

    let enum_values = context.enum_values.keys().cloned().collect::<Vec<_>>();

    let event_definitions = context
        .event_definitions
        .keys()
        .cloned()
        .collect::<Vec<_>>();

    let error_definitions = context
        .error_definitions
        .keys()
        .cloned()
        .collect::<Vec<_>>();

    let expression_statements = context
        .expression_statements
        .keys()
        .cloned()
        .collect::<Vec<_>>();

    let function_calls = context.function_calls.keys().cloned().collect::<Vec<_>>();

    let function_call_options = context
        .function_call_options
        .keys()
        .cloned()
        .collect::<Vec<_>>();

    let function_definitions = context
        .function_definitions
        .keys()
        .cloned()
        .collect::<Vec<_>>();

    let function_type_names = context
        .function_type_names
        .keys()
        .cloned()
        .collect::<Vec<_>>();

    let for_statements = context.for_statements.keys().cloned().collect::<Vec<_>>();

    let identifiers = context.identifiers.keys().cloned().collect::<Vec<_>>();

    let identifier_paths = context.identifier_paths.keys().cloned().collect::<Vec<_>>();

    let if_statements = context.if_statements.keys().cloned().collect::<Vec<_>>();

    let import_directives = context
        .import_directives
        .keys()
        .cloned()
        .collect::<Vec<_>>();

    let index_accesses = context.index_accesses.keys().cloned().collect::<Vec<_>>();

    let index_range_accesses = context
        .index_range_accesses
        .keys()
        .cloned()
        .collect::<Vec<_>>();

    let inheritance_specifiers = context
        .inheritance_specifiers
        .keys()
        .cloned()
        .collect::<Vec<_>>();

    let inline_assemblies = context
        .inline_assemblies
        .keys()
        .cloned()
        .collect::<Vec<_>>();

    let literals = context.literals.keys().cloned().collect::<Vec<_>>();

    let member_accesses = context.member_accesses.keys().cloned().collect::<Vec<_>>();

    let new_expressions = context.new_expressions.keys().cloned().collect::<Vec<_>>();

    let mappings = context.mappings.keys().cloned().collect::<Vec<_>>();

    let modifier_definitions = context
        .modifier_definitions
        .keys()
        .cloned()
        .collect::<Vec<_>>();

    let modifier_invocations = context
        .modifier_invocations
        .keys()
        .cloned()
        .collect::<Vec<_>>();

    let override_specifiers = context
        .override_specifiers
        .keys()
        .cloned()
        .collect::<Vec<_>>();

    let parameter_lists = context.parameter_lists.keys().cloned().collect::<Vec<_>>();

    let pragma_directives = context
        .pragma_directives
        .keys()
        .cloned()
        .collect::<Vec<_>>();

    let returns = context.returns.keys().cloned().collect::<Vec<_>>();

    let revert_statements = context
        .revert_statements
        .keys()
        .cloned()
        .collect::<Vec<_>>();

    let struct_definitions = context
        .struct_definitions
        .keys()
        .cloned()
        .collect::<Vec<_>>();

    let structured_documentations = context
        .structured_documentations
        .keys()
        .cloned()
        .collect::<Vec<_>>();

    let try_statements = context.try_statements.keys().cloned().collect::<Vec<_>>();

    let try_catch_clauses = context
        .try_catch_clauses
        .keys()
        .cloned()
        .collect::<Vec<_>>();

    let tuple_expressions = context
        .tuple_expressions
        .keys()
        .cloned()
        .collect::<Vec<_>>();

    let unary_operations = context.unary_operations.keys().cloned().collect::<Vec<_>>();

    let user_defined_type_names = context
        .user_defined_type_names
        .keys()
        .cloned()
        .collect::<Vec<_>>();

    let user_defined_value_type_definitions = context
        .user_defined_value_type_definitions
        .keys()
        .cloned()
        .collect::<Vec<_>>();

    let using_for_directives = context
        .using_for_directives
        .keys()
        .cloned()
        .collect::<Vec<_>>();

    let variable_declarations = context
        .variable_declarations
        .keys()
        .cloned()
        .collect::<Vec<_>>();

    let variable_declaration_statements = context
        .variable_declaration_statements
        .keys()
        .cloned()
        .collect::<Vec<_>>();

    let while_statements = context.while_statements.keys().cloned().collect::<Vec<_>>();

    all_parents.extend(array_type_names.into_iter().map(|x| x.into()));
    all_parents.extend(assignments.into_iter().map(|x| x.into()));
    all_parents.extend(binary_operations.into_iter().map(|x| x.into()));
    all_parents.extend(blocks.into_iter().map(|x| x.into()));
    all_parents.extend(conditionals.into_iter().map(|x| x.into()));
    all_parents.extend(contract_definitions.into_iter().map(|x| x.into()));
    all_parents.extend(elementary_type_names.into_iter().map(|x| x.into()));
    all_parents.extend(
        elementary_type_name_expressions
            .into_iter()
            .map(|x| x.into()),
    );
    all_parents.extend(emit_statements.into_iter().map(|x| x.into()));
    all_parents.extend(enum_definitions.into_iter().map(|x| x.into()));
    all_parents.extend(enum_values.into_iter().map(|x| x.into()));
    all_parents.extend(event_definitions.into_iter().map(|x| x.into()));
    all_parents.extend(error_definitions.into_iter().map(|x| x.into()));
    all_parents.extend(expression_statements.into_iter().map(|x| x.into()));
    all_parents.extend(function_calls.into_iter().map(|x| x.into()));
    all_parents.extend(function_call_options.into_iter().map(|x| x.into()));
    all_parents.extend(function_definitions.into_iter().map(|x| x.into()));
    all_parents.extend(function_type_names.into_iter().map(|x| x.into()));
    all_parents.extend(for_statements.into_iter().map(|x| x.into()));
    all_parents.extend(identifiers.into_iter().map(|x| x.into()));
    all_parents.extend(identifier_paths.into_iter().map(|x| x.into()));
    all_parents.extend(if_statements.into_iter().map(|x| x.into()));
    all_parents.extend(import_directives.into_iter().map(|x| x.into()));
    all_parents.extend(index_accesses.into_iter().map(|x| x.into()));
    all_parents.extend(index_range_accesses.into_iter().map(|x| x.into()));
    all_parents.extend(inheritance_specifiers.into_iter().map(|x| x.into()));
    all_parents.extend(inline_assemblies.into_iter().map(|x| x.into()));
    all_parents.extend(literals.into_iter().map(|x| x.into()));
    all_parents.extend(member_accesses.into_iter().map(|x| x.into()));
    all_parents.extend(new_expressions.into_iter().map(|x| x.into()));
    all_parents.extend(mappings.into_iter().map(|x| x.into()));
    all_parents.extend(modifier_definitions.into_iter().map(|x| x.into()));
    all_parents.extend(modifier_invocations.into_iter().map(|x| x.into()));
    all_parents.extend(override_specifiers.into_iter().map(|x| x.into()));
    all_parents.extend(parameter_lists.into_iter().map(|x| x.into()));
    all_parents.extend(pragma_directives.into_iter().map(|x| x.into()));
    all_parents.extend(returns.into_iter().map(|x| x.into()));
    all_parents.extend(revert_statements.into_iter().map(|x| x.into()));
    all_parents.extend(struct_definitions.into_iter().map(|x| x.into()));
    all_parents.extend(structured_documentations.into_iter().map(|x| x.into()));
    all_parents.extend(try_statements.into_iter().map(|x| x.into()));
    all_parents.extend(try_catch_clauses.into_iter().map(|x| x.into()));
    all_parents.extend(tuple_expressions.into_iter().map(|x| x.into()));
    all_parents.extend(unary_operations.into_iter().map(|x| x.into()));
    all_parents.extend(user_defined_type_names.into_iter().map(|x| x.into()));
    all_parents.extend(
        user_defined_value_type_definitions
            .into_iter()
            .map(|x| x.into()),
    );
    all_parents.extend(using_for_directives.into_iter().map(|x| x.into()));
    all_parents.extend(variable_declarations.into_iter().map(|x| x.into()));
    all_parents.extend(
        variable_declaration_statements
            .into_iter()
            .map(|x| x.into()),
    );
    all_parents.extend(while_statements.into_iter().map(|x| x.into()));
    all_parents
}

pub fn get_node_ids_of_ast_nodes_that_have_ids(nodes: &Vec<ASTNode>) -> HashMap<NodeID, ASTNode> {
    let mut nodes_with_ids: HashMap<NodeID, ASTNode> = HashMap::new();
    for node in nodes {
        match node {
            // ASTNode::ArrayTypeName(x) => todo: recurse
            ASTNode::Assignment(n) => nodes_with_ids.insert(n.id, node.clone()),
            ASTNode::BinaryOperation(n) => nodes_with_ids.insert(n.id, node.clone()),
            ASTNode::Block(n) => nodes_with_ids.insert(n.id, node.clone()),
            ASTNode::Conditional(n) => nodes_with_ids.insert(n.id, node.clone()),
            ASTNode::ContractDefinition(n) => nodes_with_ids.insert(n.id, node.clone()),
            // ASTNode::ElementaryTypeName(emit) => todo: recurse
            ASTNode::ElementaryTypeNameExpression(n) => nodes_with_ids.insert(n.id, node.clone()),
            // ASTNode::EmitStatement(emit) => todo: recurse
            ASTNode::EnumDefinition(n) => nodes_with_ids.insert(n.id, node.clone()),
            ASTNode::EnumValue(n) => nodes_with_ids.insert(n.id, node.clone()),
            ASTNode::EventDefinition(n) => nodes_with_ids.insert(n.id, node.clone()),
            ASTNode::ErrorDefinition(n) => nodes_with_ids.insert(n.id, node.clone()),
            // ASTNode::ExpressionStatement(_) => todo!(), todo: recurse
            ASTNode::FunctionCall(n) => nodes_with_ids.insert(n.id, node.clone()),
            ASTNode::FunctionCallOptions(n) => nodes_with_ids.insert(n.id, node.clone()),
            ASTNode::FunctionDefinition(n) => nodes_with_ids.insert(n.id, node.clone()),
            // ASTNode::FunctionTypeName(n) => nodes_with_ids.insert(n.id, node.clone()), todo: recurse
            ASTNode::ForStatement(n) => nodes_with_ids.insert(n.id, node.clone()),
            ASTNode::Identifier(n) => nodes_with_ids.insert(n.id, node.clone()),
            ASTNode::IdentifierPath(n) => nodes_with_ids.insert(n.id, node.clone()),
            ASTNode::IfStatement(n) => nodes_with_ids.insert(n.id, node.clone()),
            ASTNode::ImportDirective(n) => nodes_with_ids.insert(n.id, node.clone()),
            ASTNode::IndexAccess(n) => nodes_with_ids.insert(n.id, node.clone()),
            ASTNode::IndexRangeAccess(n) => nodes_with_ids.insert(n.id, node.clone()),
            ASTNode::InheritanceSpecifier(n) => nodes_with_ids.insert(n.id, node.clone()),
            ASTNode::InlineAssembly(n) => nodes_with_ids.insert(n.id, node.clone()),
            ASTNode::Literal(n) => nodes_with_ids.insert(n.id, node.clone()),
            ASTNode::MemberAccess(n) => nodes_with_ids.insert(n.id, node.clone()),
            ASTNode::NewExpression(n) => nodes_with_ids.insert(n.id, node.clone()),
            // ASTNode::Mapping(n) => nodes_with_ids.insert(n.id, node.clone()), todo: recurse
            ASTNode::ModifierDefinition(n) => nodes_with_ids.insert(n.id, node.clone()),
            ASTNode::ModifierInvocation(n) => nodes_with_ids.insert(n.id, node.clone()),
            ASTNode::OverrideSpecifier(n) => nodes_with_ids.insert(n.id, node.clone()),
            ASTNode::ParameterList(n) => nodes_with_ids.insert(n.id, node.clone()),
            ASTNode::PragmaDirective(n) => nodes_with_ids.insert(n.id, node.clone()),
            ASTNode::Return(n) => nodes_with_ids.insert(n.id, node.clone()),
            // ASTNode::RevertStatement(n) => nodes_with_ids.insert(n.id, node.clone()),
            ASTNode::SourceUnit(n) => nodes_with_ids.insert(n.id, node.clone()),
            ASTNode::StructDefinition(n) => nodes_with_ids.insert(n.id, node.clone()),
            ASTNode::StructuredDocumentation(n) => nodes_with_ids.insert(n.id, node.clone()),
            // ASTNode::TryStatement(n) => nodes_with_ids.insert(n.id, node.clone()),
            // ASTNode::TryCatchClause(n) => nodes_with_ids.insert(n.id, node.clone()),
            ASTNode::TupleExpression(n) => nodes_with_ids.insert(n.id, node.clone()),
            ASTNode::UnaryOperation(n) => nodes_with_ids.insert(n.id, node.clone()),
            // ASTNode::UserDefinedTypeName(n) => nodes_with_ids.insert(n.id, node.clone()),
            ASTNode::UserDefinedValueTypeDefinition(n) => nodes_with_ids.insert(n.id, node.clone()),
            ASTNode::UsingForDirective(n) => nodes_with_ids.insert(n.id, node.clone()),
            ASTNode::VariableDeclaration(n) => nodes_with_ids.insert(n.id, node.clone()),
            ASTNode::VariableDeclarationStatement(n) => nodes_with_ids.insert(n.id, node.clone()),
            ASTNode::WhileStatement(n) => nodes_with_ids.insert(n.id, node.clone()),
            _ => None,
        };
    }
    nodes_with_ids
}
