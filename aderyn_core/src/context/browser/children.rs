use crate::context::{
    browser::{
        get_all_children, get_node_ids_of_ast_nodes_that_have_ids, get_parent_chain_of_child,
    },
    workspace_context::{ASTNode, WorkspaceContext},
};

pub fn get_children_of_node(node: &ASTNode, context: &WorkspaceContext) -> Option<Vec<ASTNode>> {
    let current_node = vec![node.clone()];
    let current_nodes_id = get_node_ids_of_ast_nodes_that_have_ids(&current_node);
    if current_nodes_id.is_empty() {
        return None;
    }
    let (current_node_id, _) = current_nodes_id.iter().next().unwrap();

    let all_children = get_all_children(node);
    let all_children_ids = get_node_ids_of_ast_nodes_that_have_ids(&all_children);

    let mut immediate_children = vec![];

    for (k, v) in all_children_ids {
        let parent_chain = get_parent_chain_of_child(k, context);
        if parent_chain.len() > 1 {
            let first_parent = vec![parent_chain[1].clone()];
            let parents_id = get_node_ids_of_ast_nodes_that_have_ids(&first_parent);
            if !parents_id.is_empty() {
                let (parent_id, _) = parents_id.iter().next().unwrap();
                if parent_id == current_node_id {
                    immediate_children.push(v.clone())
                }
            }
        }
    }

    let mut hooks = vec![];

    for (idx, child) in immediate_children.iter().enumerate() {
        let trace = child.src().unwrap();
        let char_index = trace
            .split(":")
            .collect::<Vec<_>>()
            .first()
            .unwrap()
            .parse::<usize>()
            .unwrap();
        hooks.push((char_index, idx))
    }

    hooks.sort_by(|a, b| a.0.cmp(&b.0));

    let mut rearranged_children = vec![];

    for index in &hooks {
        rearranged_children.push(immediate_children[index.1].clone());
    }

    Some(rearranged_children)
}
