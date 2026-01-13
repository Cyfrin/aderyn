use super::{
    graph::{LegacyWorkspaceCallGraph, WorkspaceCallGraphs},
    router::Router,
};
use crate::{ast::*, stats::IgnoreLine};
use paste::paste;
use solidity_ast::EvmVersion;
use std::{
    collections::{HashMap, HashSet},
    path::PathBuf,
};

pub use crate::context::{ASTNode, Capturable};

/*
 * Define
 *  WorkspaceContext,
 *  NodeContext
 *
 *  impl Workspace
 */

macro_rules! create_workspace_context {
    (
        regular: $($type:ident),* $(,)?;
        yul: $($yul_type:ident),* $(,)?;
        yul_sourceless: $($yul_sourceless:ident),* $(,)?;
    ) => {
        create_workspace_context!(
            $($type),*,
            $($yul_type),*,
            $($yul_sourceless),*,
        );
    };

    ($($type:ident),* $(,)?,) => {
        paste! {
            #[derive(Default, Debug, Clone, PartialEq, Eq, Hash)]
            pub struct NodeContext {
                pub source_unit_id: NodeID,
            }

            #[derive(Default, Debug)]
            pub struct WorkspaceContext {
                pub last_source_unit_id: NodeID,
                pub parent_link: HashMap<NodeID, NodeID>,
                pub evm_version: EvmVersion,

                pub router: Option<Router>, // Function router
                pub src_filepaths: Vec<String>, // Relative source filepaths
                pub sloc_stats: HashMap<String, usize>,
                pub ignore_lines_stats: HashMap<String, Vec<IgnoreLine>>,
                pub nodes: HashMap<NodeID, ASTNode>,

                pub inward_callgraph: Option<LegacyWorkspaceCallGraph>,
                pub outward_callgraph: Option<LegacyWorkspaceCallGraph>,
                pub callgraphs: Option<WorkspaceCallGraphs>,

                pub included: HashSet<PathBuf>, // In-scope files
                pub via_ir: bool, // True if via_ir is configured for the project.

                pub source_units_context: Vec<SourceUnit>,

                $(pub(crate) [<$type:snake s_context>]: HashMap<$type, NodeContext>,)*
            }


            impl WorkspaceContext {
                $(
                    pub fn [<$type:snake s>](&self) -> Vec<&$type> {
                        self.[<$type:snake s_context>].keys().collect()
                    }
                )*

                pub fn source_units(&self) -> Vec<&SourceUnit> {
                    self.source_units_context.iter().collect()
                }

                pub fn get_source_unit_from_child_node(&self, node: &ASTNode) -> Option<&SourceUnit> {
                    let source_unit_id = match node {
                        $(ASTNode::$type(n) => self.[<$type:snake s_context>].get(n).map(|c| c.source_unit_id),)*
                        ASTNode::SourceUnit(n) => Some(n.id),
                    };
                    source_unit_id.and_then(|id| {
                        self.source_units_context.iter().find(|su| su.id == id)
                    })
                }
            }
        }
    };
}

with_node_types!(create_workspace_context);
