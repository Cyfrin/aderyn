use crate::{
    context::workspace_context::{ASTNode, WorkspaceContext},
    visitor::ast_visitor::Node,
};

use super::ExtractEverything;

pub fn get_count_of_all_children(node: &ASTNode) -> usize {
    match node {
        ASTNode::ArrayTypeName(n) => {
            let children = ExtractEverything::from(n).extracted;
            children.len()
        }
        ASTNode::Assignment(n) => {
            let children = ExtractEverything::from(n).extracted;
            children.len()
        }
        ASTNode::BinaryOperation(n) => {
            let children = ExtractEverything::from(n).extracted;
            children.len()
        }
        ASTNode::Block(n) => {
            let children = ExtractEverything::from(n).extracted;
            children.len()
        }
        ASTNode::Conditional(n) => {
            let children = ExtractEverything::from(n).extracted;
            children.len()
        }
        ASTNode::ContractDefinition(n) => {
            let children = ExtractEverything::from(n).extracted;
            children.len()
        }
        ASTNode::ElementaryTypeName(n) => {
            let children = ExtractEverything::from(n).extracted;
            children.len()
        }
        ASTNode::ElementaryTypeNameExpression(n) => {
            let children = ExtractEverything::from(n).extracted;
            children.len()
        }
        ASTNode::EmitStatement(n) => {
            let children = ExtractEverything::from(n).extracted;
            children.len()
        }
        ASTNode::EnumDefinition(n) => {
            let children = ExtractEverything::from(n).extracted;
            children.len()
        }
        ASTNode::EnumValue(n) => {
            let children = ExtractEverything::from(n).extracted;
            children.len()
        }
        ASTNode::EventDefinition(n) => {
            let children = ExtractEverything::from(n).extracted;
            children.len()
        }
        ASTNode::ErrorDefinition(n) => {
            let children = ExtractEverything::from(n).extracted;
            children.len()
        }
        ASTNode::ExpressionStatement(n) => {
            let children = ExtractEverything::from(n).extracted;
            children.len()
        }
        ASTNode::FunctionCall(n) => {
            let children = ExtractEverything::from(n).extracted;
            children.len()
        }
        ASTNode::FunctionCallOptions(n) => {
            let children = ExtractEverything::from(n).extracted;
            children.len()
        }
        ASTNode::FunctionDefinition(n) => {
            let children = ExtractEverything::from(n).extracted;
            children.len()
        }
        ASTNode::FunctionTypeName(n) => {
            let children = ExtractEverything::from(n).extracted;
            children.len()
        }
        ASTNode::ForStatement(n) => {
            let children = ExtractEverything::from(n).extracted;
            children.len()
        }
        ASTNode::Identifier(n) => {
            let children = ExtractEverything::from(n).extracted;
            children.len()
        }
        ASTNode::IdentifierPath(n) => {
            let children = ExtractEverything::from(n).extracted;
            children.len()
        }
        ASTNode::IfStatement(n) => {
            let children = ExtractEverything::from(n).extracted;
            children.len()
        }
        ASTNode::ImportDirective(n) => {
            let children = ExtractEverything::from(n).extracted;
            children.len()
        }
        ASTNode::IndexAccess(n) => {
            let children = ExtractEverything::from(n).extracted;
            children.len()
        }
        ASTNode::IndexRangeAccess(n) => {
            let children = ExtractEverything::from(n).extracted;
            children.len()
        }
        ASTNode::InheritanceSpecifier(n) => {
            let children = ExtractEverything::from(n).extracted;
            children.len()
        }
        ASTNode::InlineAssembly(n) => {
            let children = ExtractEverything::from(n).extracted;
            children.len()
        }
        ASTNode::Literal(n) => {
            let children = ExtractEverything::from(n).extracted;
            children.len()
        }
        ASTNode::MemberAccess(n) => {
            let children = ExtractEverything::from(n).extracted;
            children.len()
        }
        ASTNode::NewExpression(n) => {
            let children = ExtractEverything::from(n).extracted;
            children.len()
        }
        ASTNode::Mapping(n) => {
            let children = ExtractEverything::from(n).extracted;
            children.len()
        }
        ASTNode::ModifierDefinition(n) => {
            let children = ExtractEverything::from(n).extracted;
            children.len()
        }
        ASTNode::ModifierInvocation(n) => {
            let children = ExtractEverything::from(n).extracted;
            children.len()
        }
        ASTNode::OverrideSpecifier(n) => {
            let children = ExtractEverything::from(n).extracted;
            children.len()
        }
        ASTNode::ParameterList(n) => {
            let children = ExtractEverything::from(n).extracted;
            children.len()
        }
        ASTNode::PragmaDirective(n) => {
            let children = ExtractEverything::from(n).extracted;
            children.len()
        }
        ASTNode::Return(n) => {
            let children = ExtractEverything::from(n).extracted;
            children.len()
        }
        ASTNode::RevertStatement(n) => {
            let children = ExtractEverything::from(n).extracted;
            children.len()
        }
        ASTNode::SourceUnit(n) => {
            let children = ExtractEverything::from(n).extracted;
            children.len()
        }
        ASTNode::StructDefinition(n) => {
            let children = ExtractEverything::from(n).extracted;
            children.len()
        }
        ASTNode::StructuredDocumentation(n) => {
            let children = ExtractEverything::from(n).extracted;
            children.len()
        }
        ASTNode::TryStatement(n) => {
            let children = ExtractEverything::from(n).extracted;
            children.len()
        }
        ASTNode::TryCatchClause(n) => {
            let children = ExtractEverything::from(n).extracted;
            children.len()
        }
        ASTNode::TupleExpression(n) => {
            let children = ExtractEverything::from(n).extracted;
            children.len()
        }
        ASTNode::UnaryOperation(n) => {
            let children = ExtractEverything::from(n).extracted;
            children.len()
        }
        ASTNode::UserDefinedTypeName(n) => {
            let children = ExtractEverything::from(n).extracted;
            children.len()
        }
        ASTNode::UserDefinedValueTypeDefinition(n) => {
            let children = ExtractEverything::from(n).extracted;
            children.len()
        }
        ASTNode::UsingForDirective(n) => {
            let children = ExtractEverything::from(n).extracted;
            children.len()
        }
        ASTNode::VariableDeclaration(n) => {
            let children = ExtractEverything::from(n).extracted;
            children.len()
        }
        ASTNode::VariableDeclarationStatement(n) => {
            let children = ExtractEverything::from(n).extracted;
            children.len()
        }
        ASTNode::WhileStatement(n) => {
            let children = ExtractEverything::from(n).extracted;
            children.len()
        }
    }
}

pub fn get_parent_chain_of(node: &ASTNode, context: &WorkspaceContext) -> Option<Vec<ASTNode>> {
    let all_parents = get_all_parents_in_no_specific_order(node, context)?;
    let mut children_count = vec![];
    for (idx, parent) in all_parents.iter().enumerate() {
        let number_of_children = get_count_of_all_children(&parent);
        children_count.push((number_of_children, idx));
    }
    children_count.sort();
    let mut parent_chain = vec![];
    for (idx, _) in children_count {
        parent_chain.push(all_parents[idx].clone());
    }
    Some(parent_chain)
}

pub fn get_all_parents_in_no_specific_order(
    node: &ASTNode,
    context: &WorkspaceContext,
) -> Option<Vec<ASTNode>> {
    // Find the id of the target node
    let target_id = context.get_source_unit_from_child_node(node)?.id;

    // Gather all the parents first (not in any specific ancestral order)
    let mut all_parents: Vec<ASTNode> = vec![];

    let array_type_names = context
        .array_type_names
        .keys()
        .into_iter()
        .map(|x| x.clone())
        .collect::<Vec<_>>();

    let assignments = context
        .assignments
        .keys()
        .into_iter()
        .map(|x| x.clone())
        .collect::<Vec<_>>();

    let binary_operations = context
        .binary_operations
        .keys()
        .into_iter()
        .map(|x| x.clone())
        .collect::<Vec<_>>();

    let blocks = context
        .blocks
        .keys()
        .into_iter()
        .map(|x| x.clone())
        .collect::<Vec<_>>();

    let conditionals = context
        .conditionals
        .keys()
        .into_iter()
        .map(|x| x.clone())
        .collect::<Vec<_>>();

    let contract_definitions = context
        .contract_definitions
        .keys()
        .into_iter()
        .map(|x| x.clone())
        .collect::<Vec<_>>();

    let elementary_type_names = context
        .elementary_type_names
        .keys()
        .into_iter()
        .map(|x| x.clone())
        .collect::<Vec<_>>();

    let elementary_type_name_expressions = context
        .elementary_type_name_expressions
        .keys()
        .into_iter()
        .map(|x| x.clone())
        .collect::<Vec<_>>();

    let emit_statements = context
        .emit_statements
        .keys()
        .into_iter()
        .map(|x| x.clone())
        .collect::<Vec<_>>();

    let enum_definitions = context
        .enum_definitions
        .keys()
        .into_iter()
        .map(|x| x.clone())
        .collect::<Vec<_>>();

    let enum_values = context
        .enum_values
        .keys()
        .into_iter()
        .map(|x| x.clone())
        .collect::<Vec<_>>();

    let event_definitions = context
        .event_definitions
        .keys()
        .into_iter()
        .map(|x| x.clone())
        .collect::<Vec<_>>();

    let error_definitions = context
        .error_definitions
        .keys()
        .into_iter()
        .map(|x| x.clone())
        .collect::<Vec<_>>();

    let expression_statements = context
        .expression_statements
        .keys()
        .into_iter()
        .map(|x| x.clone())
        .collect::<Vec<_>>();

    let function_calls = context
        .function_calls
        .keys()
        .into_iter()
        .map(|x| x.clone())
        .collect::<Vec<_>>();

    let function_call_options = context
        .function_call_options
        .keys()
        .into_iter()
        .map(|x| x.clone())
        .collect::<Vec<_>>();

    let function_definitions = context
        .function_definitions
        .keys()
        .into_iter()
        .map(|x| x.clone())
        .collect::<Vec<_>>();

    let function_type_names = context
        .function_type_names
        .keys()
        .into_iter()
        .map(|x| x.clone())
        .collect::<Vec<_>>();

    let for_statements = context
        .for_statements
        .keys()
        .into_iter()
        .map(|x| x.clone())
        .collect::<Vec<_>>();

    let identifiers = context
        .identifiers
        .keys()
        .into_iter()
        .map(|x| x.clone())
        .collect::<Vec<_>>();

    let identifier_paths = context
        .identifier_paths
        .keys()
        .into_iter()
        .map(|x| x.clone())
        .collect::<Vec<_>>();

    let if_statements = context
        .if_statements
        .keys()
        .into_iter()
        .map(|x| x.clone())
        .collect::<Vec<_>>();

    let import_directives = context
        .import_directives
        .keys()
        .into_iter()
        .map(|x| x.clone())
        .collect::<Vec<_>>();

    let index_accesses = context
        .index_accesses
        .keys()
        .into_iter()
        .map(|x| x.clone())
        .collect::<Vec<_>>();

    let index_range_accesses = context
        .index_range_accesses
        .keys()
        .into_iter()
        .map(|x| x.clone())
        .collect::<Vec<_>>();

    let inheritance_specifiers = context
        .inheritance_specifiers
        .keys()
        .into_iter()
        .map(|x| x.clone())
        .collect::<Vec<_>>();

    let inline_assemblies = context
        .inline_assemblies
        .keys()
        .into_iter()
        .map(|x| x.clone())
        .collect::<Vec<_>>();

    let literals = context
        .literals
        .keys()
        .into_iter()
        .map(|x| x.clone())
        .collect::<Vec<_>>();

    let member_accesses = context
        .member_accesses
        .keys()
        .into_iter()
        .map(|x| x.clone())
        .collect::<Vec<_>>();

    let new_expressions = context
        .new_expressions
        .keys()
        .into_iter()
        .map(|x| x.clone())
        .collect::<Vec<_>>();

    let mappings = context
        .mappings
        .keys()
        .into_iter()
        .map(|x| x.clone())
        .collect::<Vec<_>>();

    let modifier_definitions = context
        .modifier_definitions
        .keys()
        .into_iter()
        .map(|x| x.clone())
        .collect::<Vec<_>>();

    let modifier_invocations = context
        .modifier_invocations
        .keys()
        .into_iter()
        .map(|x| x.clone())
        .collect::<Vec<_>>();

    let override_specifiers = context
        .override_specifiers
        .keys()
        .into_iter()
        .map(|x| x.clone())
        .collect::<Vec<_>>();

    let parameter_lists = context
        .parameter_lists
        .keys()
        .into_iter()
        .map(|x| x.clone())
        .collect::<Vec<_>>();

    let pragma_directives = context
        .pragma_directives
        .keys()
        .into_iter()
        .map(|x| x.clone())
        .collect::<Vec<_>>();

    let returns = context
        .returns
        .keys()
        .into_iter()
        .map(|x| x.clone())
        .collect::<Vec<_>>();

    let revert_statements = context
        .revert_statements
        .keys()
        .into_iter()
        .map(|x| x.clone())
        .collect::<Vec<_>>();

    let struct_definitions = context
        .struct_definitions
        .keys()
        .into_iter()
        .map(|x| x.clone())
        .collect::<Vec<_>>();

    let structured_documentations = context
        .structured_documentations
        .keys()
        .into_iter()
        .map(|x| x.clone())
        .collect::<Vec<_>>();

    let try_statements = context
        .try_statements
        .keys()
        .into_iter()
        .map(|x| x.clone())
        .collect::<Vec<_>>();

    let try_catch_clauses = context
        .try_catch_clauses
        .keys()
        .into_iter()
        .map(|x| x.clone())
        .collect::<Vec<_>>();

    let tuple_expressions = context
        .tuple_expressions
        .keys()
        .into_iter()
        .map(|x| x.clone())
        .collect::<Vec<_>>();

    let unary_operations = context
        .unary_operations
        .keys()
        .into_iter()
        .map(|x| x.clone())
        .collect::<Vec<_>>();

    let user_defined_type_names = context
        .user_defined_type_names
        .keys()
        .into_iter()
        .map(|x| x.clone())
        .collect::<Vec<_>>();

    let user_defined_value_type_definitions = context
        .user_defined_value_type_definitions
        .keys()
        .into_iter()
        .map(|x| x.clone())
        .collect::<Vec<_>>();

    let using_for_directives = context
        .using_for_directives
        .keys()
        .into_iter()
        .map(|x| x.clone())
        .collect::<Vec<_>>();

    let variable_declarations = context
        .variable_declarations
        .keys()
        .into_iter()
        .map(|x| x.clone())
        .collect::<Vec<_>>();

    let variable_declaration_statements = context
        .variable_declaration_statements
        .keys()
        .into_iter()
        .map(|x| x.clone())
        .collect::<Vec<_>>();

    let while_statements = context
        .while_statements
        .keys()
        .into_iter()
        .map(|x| x.clone())
        .collect::<Vec<_>>();

    all_parents.extend(get_parents_of_child_with_id_from(
        target_id,
        context,
        array_type_names,
    )?);
    all_parents.extend(get_parents_of_child_with_id_from(
        target_id,
        context,
        assignments,
    )?);
    all_parents.extend(get_parents_of_child_with_id_from(
        target_id,
        context,
        binary_operations,
    )?);
    all_parents.extend(get_parents_of_child_with_id_from(
        target_id, context, blocks,
    )?);
    all_parents.extend(get_parents_of_child_with_id_from(
        target_id,
        context,
        conditionals,
    )?);
    all_parents.extend(get_parents_of_child_with_id_from(
        target_id,
        context,
        contract_definitions,
    )?);
    all_parents.extend(get_parents_of_child_with_id_from(
        target_id,
        context,
        elementary_type_names,
    )?);
    all_parents.extend(get_parents_of_child_with_id_from(
        target_id,
        context,
        elementary_type_name_expressions,
    )?);
    all_parents.extend(get_parents_of_child_with_id_from(
        target_id,
        context,
        emit_statements,
    )?);
    all_parents.extend(get_parents_of_child_with_id_from(
        target_id,
        context,
        enum_definitions,
    )?);
    all_parents.extend(get_parents_of_child_with_id_from(
        target_id,
        context,
        enum_values,
    )?);
    all_parents.extend(get_parents_of_child_with_id_from(
        target_id,
        context,
        event_definitions,
    )?);
    all_parents.extend(get_parents_of_child_with_id_from(
        target_id,
        context,
        error_definitions,
    )?);
    all_parents.extend(get_parents_of_child_with_id_from(
        target_id,
        context,
        expression_statements,
    )?);
    all_parents.extend(get_parents_of_child_with_id_from(
        target_id,
        context,
        function_calls,
    )?);
    all_parents.extend(get_parents_of_child_with_id_from(
        target_id,
        context,
        function_call_options,
    )?);
    all_parents.extend(get_parents_of_child_with_id_from(
        target_id,
        context,
        function_definitions,
    )?);
    all_parents.extend(get_parents_of_child_with_id_from(
        target_id,
        context,
        function_type_names,
    )?);
    all_parents.extend(get_parents_of_child_with_id_from(
        target_id,
        context,
        for_statements,
    )?);
    all_parents.extend(get_parents_of_child_with_id_from(
        target_id,
        context,
        identifiers,
    )?);
    all_parents.extend(get_parents_of_child_with_id_from(
        target_id,
        context,
        identifier_paths,
    )?);
    all_parents.extend(get_parents_of_child_with_id_from(
        target_id,
        context,
        if_statements,
    )?);
    all_parents.extend(get_parents_of_child_with_id_from(
        target_id,
        context,
        import_directives,
    )?);
    all_parents.extend(get_parents_of_child_with_id_from(
        target_id,
        context,
        index_accesses,
    )?);
    all_parents.extend(get_parents_of_child_with_id_from(
        target_id,
        context,
        index_range_accesses,
    )?);
    all_parents.extend(get_parents_of_child_with_id_from(
        target_id,
        context,
        inheritance_specifiers,
    )?);
    all_parents.extend(get_parents_of_child_with_id_from(
        target_id,
        context,
        inline_assemblies,
    )?);
    all_parents.extend(get_parents_of_child_with_id_from(
        target_id, context, literals,
    )?);
    all_parents.extend(get_parents_of_child_with_id_from(
        target_id,
        context,
        member_accesses,
    )?);
    all_parents.extend(get_parents_of_child_with_id_from(
        target_id,
        context,
        new_expressions,
    )?);
    all_parents.extend(get_parents_of_child_with_id_from(
        target_id, context, mappings,
    )?);
    all_parents.extend(get_parents_of_child_with_id_from(
        target_id,
        context,
        modifier_definitions,
    )?);
    all_parents.extend(get_parents_of_child_with_id_from(
        target_id,
        context,
        modifier_invocations,
    )?);
    all_parents.extend(get_parents_of_child_with_id_from(
        target_id,
        context,
        override_specifiers,
    )?);
    all_parents.extend(get_parents_of_child_with_id_from(
        target_id,
        context,
        parameter_lists,
    )?);
    all_parents.extend(get_parents_of_child_with_id_from(
        target_id,
        context,
        pragma_directives,
    )?);
    all_parents.extend(get_parents_of_child_with_id_from(
        target_id, context, returns,
    )?);
    all_parents.extend(get_parents_of_child_with_id_from(
        target_id,
        context,
        revert_statements,
    )?);
    all_parents.extend(get_parents_of_child_with_id_from(
        target_id,
        context,
        struct_definitions,
    )?);
    all_parents.extend(get_parents_of_child_with_id_from(
        target_id,
        context,
        structured_documentations,
    )?);
    all_parents.extend(get_parents_of_child_with_id_from(
        target_id,
        context,
        try_statements,
    )?);
    all_parents.extend(get_parents_of_child_with_id_from(
        target_id,
        context,
        try_catch_clauses,
    )?);
    all_parents.extend(get_parents_of_child_with_id_from(
        target_id,
        context,
        tuple_expressions,
    )?);
    all_parents.extend(get_parents_of_child_with_id_from(
        target_id,
        context,
        unary_operations,
    )?);
    all_parents.extend(get_parents_of_child_with_id_from(
        target_id,
        context,
        user_defined_type_names,
    )?);
    all_parents.extend(get_parents_of_child_with_id_from(
        target_id,
        context,
        user_defined_value_type_definitions,
    )?);
    all_parents.extend(get_parents_of_child_with_id_from(
        target_id,
        context,
        using_for_directives,
    )?);
    all_parents.extend(get_parents_of_child_with_id_from(
        target_id,
        context,
        variable_declarations,
    )?);
    all_parents.extend(get_parents_of_child_with_id_from(
        target_id,
        context,
        variable_declaration_statements,
    )?);
    all_parents.extend(get_parents_of_child_with_id_from(
        target_id,
        context,
        while_statements,
    )?);

    Some(all_parents)
}

pub fn get_parents_of_child_with_id_from<T: Node>(
    child_id: i64,
    context: &WorkspaceContext,
    nodes: Vec<T>,
) -> Option<Vec<ASTNode>> {
    // Gather all the parents first (not in any specific ancestral order)
    let mut all_parents: Vec<ASTNode> = vec![];

    // Find all the nodes where the extracted children contains target node id
    for node in nodes {
        let children = ExtractEverything::from(&node).extracted;
        for child in &children {
            let id = context.get_source_unit_from_child_node(child)?.id;
            if child_id == id {
                all_parents.push(child.clone());
            }
        }
    }

    Some(all_parents)
}
