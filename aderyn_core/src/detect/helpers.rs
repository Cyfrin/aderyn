use crate::{ast::NodeID, context::workspace_context::WorkspaceContext};

/// Count the number of identifiers that reference a given ID in the context.
pub fn count_identifiers_that_reference_an_id(
    context: &WorkspaceContext,
    function_id: NodeID,
) -> i32 {
    let mut count = 0;
    context.identifiers().iter().for_each(|&identifier| {
        if identifier.referenced_declaration == function_id {
            count += 1;
        }
    });
    count
}
