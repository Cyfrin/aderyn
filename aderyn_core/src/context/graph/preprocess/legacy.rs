use std::collections::{hash_map, HashMap, HashSet};

use crate::{
    ast::{Expression, IdentifierOrIdentifierPath, NodeID, NodeType},
    context::{
        browser::{ExtractFunctionCalls, ExtractModifierInvocations},
        workspace::WorkspaceContext,
    },
};

use crate::context::graph::{
    traits::Transpose, Error, LegacyWorkspaceCallGraph, RawCallGraph, Result,
};

impl LegacyWorkspaceCallGraph {
    /// Formula to create [`WorkspaceCallGraph`] for global preprocessing .
    pub fn from_context(context: &WorkspaceContext) -> Result<LegacyWorkspaceCallGraph> {
        let mut raw_callgraph: RawCallGraph = HashMap::new();
        let mut visited: HashSet<NodeID> = HashSet::new();

        let funcs = context
            .function_definitions()
            .into_iter()
            .filter(|func| func.implemented)
            .collect::<Vec<_>>();

        let modifier_definitions = context.modifier_definitions();

        for func in funcs {
            dfs_to_create_graph(func.id, &mut raw_callgraph, &mut visited, context)
                .map_err(|_| Error::WorkspaceCallGraphDFSError)?;
        }

        for modifier in modifier_definitions {
            dfs_to_create_graph(modifier.id, &mut raw_callgraph, &mut visited, context)
                .map_err(|_| Error::WorkspaceCallGraphDFSError)?;
        }

        Ok(LegacyWorkspaceCallGraph { raw_callgraph })
    }
}

/// Make connections from each of the nodes of [`crate::ast::FunctionDefinition`] and
/// [`crate::ast::ModifierDefinition`] with their connected counterparts.
fn dfs_to_create_graph(
    id: NodeID,
    raw_callgraph: &mut RawCallGraph,
    visited: &mut HashSet<NodeID>,
    context: &WorkspaceContext,
) -> Result<()> {
    if visited.contains(&id) {
        return Ok(());
    }

    visited.insert(id);

    // Only deal with `id`s that are in scope right now
    if let Some(from_node) = context.nodes.get(&id) {
        // referenced_declarations from previous calls in the recursion stack need to be vetted
        if from_node.node_type() != NodeType::FunctionDefinition
            && from_node.node_type() != NodeType::ModifierDefinition
        {
            return Ok(());
        }

        // connections to FunctionDefinition
        let function_calls = ExtractFunctionCalls::from(from_node).extracted;
        for function_call in function_calls {
            if let Expression::Identifier(identifier) = function_call.expression.as_ref() {
                if let Some(referenced_function_id) = identifier.referenced_declaration {
                    create_connection_if_not_exists(id, referenced_function_id, raw_callgraph);
                    dfs_to_create_graph(referenced_function_id, raw_callgraph, visited, context)?;
                }
            }
        }

        // connections to ModifierDefinition
        let modifier_invocations = ExtractModifierInvocations::from(from_node).extracted;
        for modifier_invocation in &modifier_invocations {
            match &modifier_invocation.modifier_name {
                IdentifierOrIdentifierPath::Identifier(identifier) => {
                    if let Some(reference_modifier_id) = identifier.referenced_declaration {
                        create_connection_if_not_exists(id, reference_modifier_id, raw_callgraph);
                        dfs_to_create_graph(
                            reference_modifier_id,
                            raw_callgraph,
                            visited,
                            context,
                        )?;
                    }
                }
                IdentifierOrIdentifierPath::IdentifierPath(identifier_path) => {
                    let referenced_modifier_id = identifier_path.referenced_declaration;
                    create_connection_if_not_exists(id, referenced_modifier_id, raw_callgraph);
                    dfs_to_create_graph(referenced_modifier_id, raw_callgraph, visited, context)?;
                }
            }
        }
    }

    // Change the default return to error later in "strict mode" maybe, because if we
    // can't find the node that means, the file was not in scope and hence it is not
    // available in the context although references to it exist.
    Ok(())
}

fn create_connection_if_not_exists(
    from_id: NodeID,
    to_id: NodeID,
    raw_callgraph: &mut RawCallGraph,
) {
    match raw_callgraph.entry(from_id) {
        hash_map::Entry::Occupied(mut o) => {
            // Performance Tip: Maybe later use binary search (it requires keeping ascending order
            // while inserting tho)
            if !o.get().contains(&to_id) {
                o.get_mut().push(to_id);
            }
        }
        hash_map::Entry::Vacant(v) => {
            v.insert(vec![to_id]);
        }
    }
}

impl Transpose for RawCallGraph {
    fn reverse(&self) -> Self {
        let mut reversed_callgraph = RawCallGraph::default();
        for (from_id, tos) in self {
            for to_id in tos {
                create_connection_if_not_exists(*to_id, *from_id, &mut reversed_callgraph);
            }
        }
        reversed_callgraph
    }
}
