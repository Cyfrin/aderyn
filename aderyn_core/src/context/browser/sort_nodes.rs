use std::collections::BTreeSet;

use crate::context::workspace_context::{ASTNode, WorkspaceContext};

pub trait SortNodeReferencesToSequence<'a> {
    fn sort_by_src_position(self, context: &'a WorkspaceContext) -> Option<Vec<&'a ASTNode>>;
}

pub trait SortOwnedNodesToSequence<'a> {
    fn sort_by_src_position(self, context: &'a WorkspaceContext) -> Option<Vec<ASTNode>>;
}

impl<'a> SortNodeReferencesToSequence<'a> for &[&'a ASTNode] {
    fn sort_by_src_position(self, context: &'a WorkspaceContext) -> Option<Vec<&'a ASTNode>> {
        sort_by_src_position(self, context)
    }
}

impl<'a> SortOwnedNodesToSequence<'a> for &[ASTNode] {
    fn sort_by_src_position(self, context: &'a WorkspaceContext) -> Option<Vec<ASTNode>> {
        let nodes = self.iter().collect::<Vec<_>>();
        let sorted = sort_by_src_position(&nodes, context);
        if let Some(sorted_nodes) = sorted {
            let owned_nodes = sorted_nodes.iter().map(|&x| x.clone()).collect::<Vec<_>>();
            return Some(owned_nodes);
        }
        None
    }
}

fn sort_by_src_position<'a>(
    nodes: &[&'a ASTNode],
    context: &'a WorkspaceContext,
) -> Option<Vec<&'a ASTNode>> {
    if !nodes.iter().all(|x| x.id().is_some()) {
        return None;
    }

    // Make sure all these nodes belong to the same file
    let c = nodes
        .iter()
        .map(|x| {
            let key = context.get_node_sort_key_pure(x);
            key.0 // src location
        })
        .collect::<BTreeSet<_>>();

    if c.len() != 1 && c.first() != Some(&String::from("")) {
        return None;
    }

    // Now sort them
    let mut nodes = nodes.to_vec();
    nodes.sort_by(|a, b| {
        context
            .get_relative_location_of_nodes(a.id().unwrap(), b.id().unwrap())
            .unwrap()
    });
    Some(nodes)
}
