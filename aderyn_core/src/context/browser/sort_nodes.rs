use std::collections::BTreeSet;

use crate::context::workspace_context::{ASTNode, WorkspaceContext};

pub trait SortNodeReferencesToSequence<'a> {
    fn sequence_sort(self, context: &'a WorkspaceContext) -> Option<Vec<&'a ASTNode>>;
}

pub trait SortOwnedNodesToSequence<'a> {
    fn sequence_sort(self, context: &'a WorkspaceContext) -> Option<Vec<ASTNode>>;
}

impl<'a> SortNodeReferencesToSequence<'a> for &mut [&'a ASTNode] {
    fn sequence_sort(self, context: &'a WorkspaceContext) -> Option<Vec<&'a ASTNode>> {
        sequence_sort(self, context)
    }
}

impl<'a> SortOwnedNodesToSequence<'a> for &mut [ASTNode] {
    fn sequence_sort(self, context: &'a WorkspaceContext) -> Option<Vec<ASTNode>> {
        let mut nodes = self.iter().map(|x| x).collect::<Vec<_>>();
        let sorted = sequence_sort(&mut nodes, context);
        if let Some(sorted_nodes) = sorted {
            let owned_nodes = sorted_nodes.iter().map(|&x| x.clone()).collect::<Vec<_>>();
            return Some(owned_nodes);
        }
        None
    }
}

fn sequence_sort<'a>(
    nodes: &mut [&'a ASTNode],
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
    let mut nodes: Vec<_> = nodes.iter().map(|x| *x).collect();
    nodes.sort_by(|a, b| {
        context
            .get_relative_location_of_nodes(a.id().unwrap(), b.id().unwrap())
            .unwrap()
    });
    Some(nodes)
}
