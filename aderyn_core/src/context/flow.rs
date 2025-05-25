pub mod display;
pub mod error;
pub mod kind;
pub mod primitives;
pub mod reducibles;
mod tests;
pub mod utils;
pub mod visualizer;
pub mod voids;

use crate::{
    ast::*,
    context::flow::utils::{discover_jump_sources, Calibration},
};

pub use kind::CfgNodeKind;
pub use reducibles::CfgBlock;

use std::collections::{hash_map::Entry, HashMap, HashSet, VecDeque};

use self::{
    primitives::*,
    reducibles::{
        CfgDoWhileStatement, CfgForStatement, CfgIfStatement, CfgUncheckedBlock, CfgWhileStatement,
    },
    utils::find_jump_dest,
    voids::{CfgEndNode, CfgStartNode},
};

use super::workspace::WorkspaceContext;

// This is done to differentiate AstNodeIDs from CfgNodeIDs
type AstNodeId = NodeID;

#[derive(Eq, Hash, Default, PartialEq, Clone, Copy, Debug, PartialOrd, Ord)]
pub struct CfgNodeId(usize);

impl CfgNodeId {
    #[allow(dead_code)]
    fn peek(&self) -> usize {
        self.0
    }
    #[allow(dead_code)]
    fn peek_mut(&mut self) -> &mut usize {
        &mut self.0
    }
    fn advance(&mut self) {
        self.0 += 1;
    }
}

#[derive(Debug, Clone)]
pub enum CfgNodeDescriptor {
    // Void nodes
    Start(Box<CfgStartNode>),
    End(Box<CfgEndNode>),

    // Primitives
    VariableDeclarationStatement(Box<CfgVariableDeclarationStatement>),
    ExpressionStatement(Box<CfgExpressionStatement>),
    PlaceholderStatement(Box<CfgPlaceholderStatement>),
    Break(Box<CfgBreakStatement>),
    Continue(Box<CfgContinueStatement>),
    Return(Box<CfgReturnStatement>),
    EmitStatement(Box<CfgEmitStatement>),
    RevertStatement(Box<CfgRevertStatement>),
    TryStatement(Box<CfgTryStatement>),
    InlineAssembly(Box<CfgInlineAssemblyStatement>),
    IfStatementCondition(Box<CfgIfStatementCondition>),
    WhileStatementCondition(Box<CfgWhileStatementCondition>),
    ForStatementCondition(Box<CfgForStatementCondition>),
    DoWhileStatementCondition(Box<CfgDoWhileStatementCondition>),

    // Reducibles
    Block(Box<CfgBlock>),
    UncheckedBlock(Box<CfgUncheckedBlock>),
    IfStatement(Box<CfgIfStatement>),
    WhileStatement(Box<CfgWhileStatement>),
    ForStatement(Box<CfgForStatement>),
    DoWhileStatement(Box<CfgDoWhileStatement>),
}

#[derive(Debug, Clone)]
pub struct CfgNode {
    /// Node ID
    pub id: CfgNodeId,
    /// Node descriptor
    pub nd: CfgNodeDescriptor,
}

/// Control fow graph
#[derive(Default, Debug)]
pub struct Cfg {
    /// Node registry
    pub nodes: HashMap<CfgNodeId, CfgNode>,

    /// Adjacency list representation of the Control Flow Graph
    pub adj_list: HashMap<CfgNodeId, Vec<CfgNodeId>>,

    /// ID to construct the next node
    next_available_id: CfgNodeId,

    /// Current reducibles
    reduction_queue: VecDeque<CfgNodeId>,

    /// Lookup the corresponding End node for any start node (Start*, End*)
    start_end_pairs: HashMap<CfgNodeId, CfgNodeId>,

    /// Lookup the Condition node for a given loop's start node
    start_cond_pairs: HashMap<CfgNodeId, CfgNodeId>,

    /// Lookup the loop_expression node for a given for loop's start node
    start_loop_expr: HashMap<CfgNodeId, CfgNodeId>,

    /// Lookup the StartFor given end of loop_expr
    loop_expr_start: HashMap<CfgNodeId, CfgNodeId>,

    /// Lookup the StartWhile, StartDoWhile of a loop condition given condition
    cond_start_pairs: HashMap<CfgNodeId, CfgNodeId>,
}

pub trait CfgReduce {
    fn reduce(&self, context: &WorkspaceContext, cfg: &mut Cfg) -> (CfgNodeId, CfgNodeId);
}

impl Cfg {
    pub fn new() -> Self {
        Default::default()
    }
    pub fn total_nodes(&self) -> usize {
        debug_assert_eq!(self.next_available_id.peek(), self.nodes.len());
        self.next_available_id.peek()
    }
    pub fn total_edges(&self) -> usize {
        self.adj_list.values().map(|conn| conn.len()).sum()
    }
    fn add_raw_node(&mut self, id: CfgNodeId, nd: CfgNodeDescriptor) {
        let cfg_node = CfgNode { id, nd };
        self.nodes.insert(id, cfg_node);
    }
    fn add_raw_directed_edge(&mut self, from: CfgNodeId, to: CfgNodeId) {
        match self.adj_list.entry(from) {
            Entry::Vacant(v) => {
                v.insert(vec![to]);
            }
            Entry::Occupied(mut o) => {
                o.get_mut().push(to);
            }
        };
    }
    fn remove_raw_directed_edge(&mut self, from: CfgNodeId, to: CfgNodeId) {
        let existing_nodes = self.adj_list.get_mut(&from).expect("Relationship doesn't exist");
        existing_nodes.retain_mut(|x| *x != to);
    }
    fn raw_predecessors(&self, id: CfgNodeId) -> Vec<CfgNodeId> {
        let mut predecessors = vec![];
        for (from, to_list) in &self.adj_list {
            if to_list.contains(&id) {
                predecessors.push(*from);
            }
        }
        predecessors
    }
    fn raw_successors(&self, id: CfgNodeId) -> Vec<CfgNodeId> {
        let Some(successors) = self.adj_list.get(&id) else {
            return Default::default();
        };
        successors.to_vec()
    }
    fn remove_raw_edges_involving(&mut self, node_id: CfgNodeId) {
        // Remove all successors' edges starting from node_id
        self.adj_list.remove(&node_id);

        // Remove edges ending at node_id
        for (_, to_list) in self.adj_list.iter_mut() {
            to_list.retain_mut(|x| *x != node_id);
        }
    }
    fn reset_raw_successors(&mut self, node_id: CfgNodeId, to: CfgNodeId) {
        // Remove edges starting from node_id
        self.adj_list.remove(&node_id);
        self.add_raw_directed_edge(node_id, to);
    }
}

impl Cfg {
    /// Assigns a unique ID to the given node and adds it to the CFG
    #[must_use]
    pub fn add_node(&mut self, nd: CfgNodeDescriptor) -> CfgNodeId {
        // Grab the currently available id
        let node_id = self.next_available_id;

        // Increment the ID for next use
        self.next_available_id.advance();

        // Check that node with that ID has not already been inserted
        assert!(!self.nodes.contains_key(&node_id));

        // Queue the node to be reduced if it is reducible
        if nd.kind() == CfgNodeKind::Reducible {
            self.reduction_queue.push_back(node_id);
        }

        // Add the node to the CFG
        self.add_raw_node(node_id, nd);

        //Maintain map from ast to cfg
        //if let Some(ast_id) = nd.reflect() {
        //    self.ast_to_cfg.insert(ast_id, node_id);
        //}

        // Check that node has been inserted
        assert!(self.nodes.contains_key(&node_id));

        // Return the ID of the freshly inserted node
        node_id
    }

    /// Disconnects existing relationships (mostly used during reduction)
    pub fn remove_flow_edge(&mut self, from: CfgNodeId, to: CfgNodeId) {
        self.remove_raw_directed_edge(from, to);
    }

    /// Connects the given two given nodes in the CFG
    pub fn add_flow_edge(&mut self, from: CfgNodeId, to: CfgNodeId) {
        self.add_raw_directed_edge(from, to);
    }

    /// Reduce the reducible nodes stored the queue at the time of adding nodes
    pub fn reduce(&mut self, context: &WorkspaceContext, reduction_candidate: CfgNodeId) {
        // Step 0: Remove the node that's being reduced
        let cfg_node =
            self.nodes.remove(&reduction_candidate).expect("Reduction candidate doesn't exist");

        // Step 1: Get the predecessors
        let predecessors = self.raw_predecessors(reduction_candidate);

        // Step 2: Get the successors
        let successors = self.raw_successors(reduction_candidate);

        // Step 3: Remove existing predecessor relationships with reduction candidate to build new
        // ones
        for pred in &predecessors {
            self.remove_flow_edge(*pred, cfg_node.id);
        }

        // Step 4: Remove existing predecessor relationships with reduction candidate to build new
        // ones
        for succ in &successors {
            self.remove_flow_edge(cfg_node.id, *succ);
        }

        // Step 5: Get the (start s, end e) of the reduced cfg
        let (start_id, end_id) = match cfg_node.nd {
            // Voids and Primitives
            CfgNodeDescriptor::Start(_)
            | CfgNodeDescriptor::End(_)
            | CfgNodeDescriptor::VariableDeclarationStatement(_)
            | CfgNodeDescriptor::Break(_)
            | CfgNodeDescriptor::Return(_)
            | CfgNodeDescriptor::Continue(_)
            | CfgNodeDescriptor::RevertStatement(_)
            | CfgNodeDescriptor::PlaceholderStatement(_)
            | CfgNodeDescriptor::InlineAssembly(_)
            | CfgNodeDescriptor::TryStatement(_)
            | CfgNodeDescriptor::EmitStatement(_)
            | CfgNodeDescriptor::ExpressionStatement(_)
            | CfgNodeDescriptor::WhileStatementCondition(_)
            | CfgNodeDescriptor::ForStatementCondition(_)
            | CfgNodeDescriptor::DoWhileStatementCondition(_)
            | CfgNodeDescriptor::IfStatementCondition(_) => {
                unreachable!("Expect only reducible nodes")
            }

            // Reducibles
            CfgNodeDescriptor::Block(cfg_block) => cfg_block.reduce(context, self),
            CfgNodeDescriptor::UncheckedBlock(cfg_block) => cfg_block.reduce(context, self),
            CfgNodeDescriptor::IfStatement(cfg_block) => cfg_block.reduce(context, self),
            CfgNodeDescriptor::WhileStatement(cfg_block) => cfg_block.reduce(context, self),
            CfgNodeDescriptor::ForStatement(cfg_block) => cfg_block.reduce(context, self),
            CfgNodeDescriptor::DoWhileStatement(cfg_block) => cfg_block.reduce(context, self),
        };

        // Step 6: Connect all the predecessors to `s`
        for pred in &predecessors {
            self.add_flow_edge(*pred, start_id);
        }

        // Step 7: Connect `e` to all the successors
        for succ in &successors {
            self.add_flow_edge(end_id, *succ);
        }

        // Step 8: Remove existing connections with reduction_candidate
        self.remove_raw_edges_involving(reduction_candidate);
    }

    /// Corrects the flow of continue, break and return statements
    ///
    /// This is hard to perform at the time of reduction so it must be done post-reduction.
    ///
    /// Continue CFG Nodes should flow back the parent loop's condition node in case of a `while` or
    /// `do while` and likewise to the the parent loop's update expression in case of `for`.
    ///
    /// Break CFG Nodes should always flow to the end of the parent loop
    ///
    /// Return CFG Nodes should flow to the end of the function body or a modifier body
    ///
    /// Arguments
    ///
    /// * start_node - Node discovery starts here at this point.
    ///
    /// * end_node - Return statements flow to here. It also serves as a fallback for break and
    ///   continue statements if parent loop is not found
    pub fn calibrate_jump_statements_in_body(
        &mut self,
        start_node: CfgNodeId,
        end_node: CfgNodeId,
    ) {
        // Jump sources
        let mut continue_statements = vec![];
        let mut break_statements = vec![];
        let mut return_statements = vec![];

        let mut visited: HashSet<CfgNodeId> = Default::default();

        // Start node sets the scope of discovery
        discover_jump_sources(
            self,
            start_node,
            &mut visited,
            &mut continue_statements,
            &mut break_statements,
            &mut return_statements,
        );

        // Proposed Calibrations
        let mut proposed_calibrations = vec![];

        for continue_statement in continue_statements {
            let mut visited = Default::default();
            let dest = find_jump_dest(self, continue_statement, &mut visited).unwrap_or_default();
            proposed_calibrations.push(Calibration::ContinueShouldFlowTo(continue_statement, dest));
        }

        for break_statement in break_statements {
            let mut visited = Default::default();
            let dest = find_jump_dest(self, break_statement, &mut visited).unwrap_or_default();
            proposed_calibrations.push(Calibration::BreakShouldFlowTo(break_statement, dest));
        }

        for return_statement in return_statements {
            proposed_calibrations.push(Calibration::ReturnShouldFlowToEndNode(return_statement));
        }

        // End node now comes into play
        self.calibrate(proposed_calibrations, end_node);
    }

    pub fn find_ending_counter_part(&self, start_node_id: CfgNodeId) -> CfgNodeId {
        *self.start_end_pairs.get(&start_node_id).expect("ending counter part not found!")
    }

    pub fn find_condition_node(&self, start_loop_id: CfgNodeId) -> CfgNodeId {
        *self.start_cond_pairs.get(&start_loop_id).expect("could not resolve condition!")
    }

    pub fn find_loop_expression_node(&self, start_loop_id: CfgNodeId) -> CfgNodeId {
        *self.start_loop_expr.get(&start_loop_id).expect("could not resolve loop_expression!")
    }
}

impl Cfg {
    /// Creates a new CFG from a given FunctionDefinition's body
    ///
    /// * Returns - Tuple containing Cfg, Start Node, End Node
    ///
    /// We don't yet have the ability to derive a CFG for the whole function because that involves
    /// combining modifiers with the function body plus resolving internal functions, etc.
    /// That's why the name here is from_function_body. We only create a CFG for the function's
    /// body. It is static.
    pub fn from_function_body(
        context: &WorkspaceContext,
        function_definition: &FunctionDefinition,
    ) -> Option<(Cfg, CfgNodeId, CfgNodeId)> {
        // Verify that the function has a body
        let function_body_block = function_definition.body.as_ref()?;

        // Create an empty Cfg
        let mut cfg = Self::new();

        // Add the starters for function body
        let start = cfg.add_start_function_body_node(function_definition.id);
        let end = cfg.add_end_function_body_node(function_definition.id);
        cfg.start_end_pairs.insert(start, end);

        // Add the actual function body
        let block = cfg.add_block_node(function_body_block);

        // Connect the flow edges
        cfg.add_flow_edge(start, block);
        cfg.add_flow_edge(block, end);

        // Reduction step (Standard thing to do after you assemble your Cfg skeleton)
        while let Some(reduction_candidate) = cfg.reduction_queue.pop_front() {
            cfg.reduce(context, reduction_candidate);
        }

        // Calibration step (Standard thing to do after you reduce your CFG)
        cfg.calibrate_jump_statements_in_body(start, end);

        // Return the CFG
        Some((cfg, start, end))
    }

    /// Same as [`Self::from_function_body`] but for modifiers
    pub fn from_modifier_body(
        context: &WorkspaceContext,
        modifier_definition: &ModifierDefinition,
    ) -> Option<(Cfg, CfgNodeId, CfgNodeId)> {
        // Verify that the function has a body
        let Some(modifier_body_block) = &modifier_definition.body else {
            return None;
        };

        // Create an empty Cfg
        let mut cfg = Self::new();

        // Add the starters for function body
        let start = cfg.add_start_modifier_body_node(modifier_definition.id);
        let end = cfg.add_end_modifier_body_node(modifier_definition.id);
        cfg.start_end_pairs.insert(start, end);

        // Add the actual function body
        let block = cfg.add_block_node(modifier_body_block);

        // Connect the flow edges
        cfg.add_flow_edge(start, block);
        cfg.add_flow_edge(block, end);

        // Reduction step (Standard thing to do after you assemble your Cfg skeleton)
        while let Some(reduction_candidate) = cfg.reduction_queue.pop_front() {
            cfg.reduce(context, reduction_candidate);
        }

        // Calibration step (Standard thing to do after you reduce your CFG)
        cfg.calibrate_jump_statements_in_body(start, end);

        // Return the CFG
        Some((cfg, start, end))
    }
}

// These methods help with recursion for detectors using the library
impl CfgNodeId {
    pub fn children(&self, cfg: &Cfg) -> Vec<CfgNodeId> {
        cfg.raw_successors(*self)
    }
}

impl CfgNode {
    pub fn children<'a>(&self, cfg: &'a Cfg) -> Vec<&'a CfgNode> {
        let children_ids = cfg.raw_successors(self.id);
        children_ids.into_iter().map(|c| cfg.nodes.get(&c).expect("cfg invalid!")).collect()
    }
}
