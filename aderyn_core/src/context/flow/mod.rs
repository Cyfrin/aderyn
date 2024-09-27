pub mod error;
pub mod kind;
pub mod primitives;
pub mod reducibles;

use crate::ast::*;

pub use kind::CfgNodeKind;
pub use reducibles::CfgBlock;

use std::collections::{hash_map::Entry, HashMap, VecDeque};

// This is done to differentiate AstNodeIDs from CfgNodeIDs
type AstNodeId = NodeID;

#[derive(Eq, Hash, Default, PartialEq, Clone, Copy, Debug)]
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
    Start,
    End,

    // Primitives
    VariableDeclarationStatement,
    ExpressionStatement,

    // Reducibles
    Block(Box<CfgBlock>),
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
}

pub trait CfgReduce {
    fn reduce(&self, cfg: &mut Cfg) -> (CfgNodeId, CfgNodeId);
}

impl Cfg {
    pub fn new() -> Self {
        Default::default()
    }
    pub fn total_nodes(&self) -> usize {
        debug_assert_eq!(self.next_available_id.peek(), self.nodes.len());
        self.next_available_id.peek()
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
        let existing_nodes = self
            .adj_list
            .get_mut(&from)
            .expect("Relationship doesn't exist");
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

        // Check that node has been inserted
        assert!(self.nodes.contains_key(&node_id));

        // Return the ID of the freshly inserted node
        node_id
    }

    /// Disconnects existing relationships (mostly used during reduction)
    pub fn remove_flow_edge(&mut self, from: CfgNodeId, to: CfgNodeId) {
        self.remove_raw_directed_edge(from, to)
    }

    /// Connects the given two given nodes in the CFG
    pub fn add_flow_edge(&mut self, from: CfgNodeId, to: CfgNodeId) {
        self.add_raw_directed_edge(from, to);
    }

    /// Reduce the reducible nodes stored the queue at the time of adding nodes
    pub fn reduce(&mut self, reduction_candidate: CfgNodeId) {
        // Step 0: Remove the node that's being reduced
        let cfg_node = self
            .nodes
            .remove(&reduction_candidate)
            .expect("Reduction candidate doesn't exist");

        // Step 1: Get the predecessors
        let predecessors = self.raw_predecessors(reduction_candidate);

        // Step 2: Get the successors
        let successors = self.raw_successors(reduction_candidate);

        // Step 3: Remove existing predecessor relationships with reduction candidate to build new ones
        for pred in &predecessors {
            self.remove_flow_edge(*pred, cfg_node.id);
        }

        // Step 4: Remove existing predecessor relationships with reduction candidate to build new ones
        for succ in &successors {
            self.remove_flow_edge(cfg_node.id, *succ);
        }

        // Step 5: Get the (start s, end e) of the reduced cfg
        let (start_id, end_id) = match cfg_node.nd {
            // Voids and Primitives
            CfgNodeDescriptor::Start
            | CfgNodeDescriptor::End
            | CfgNodeDescriptor::VariableDeclarationStatement
            | CfgNodeDescriptor::ExpressionStatement => unreachable!("Expect only reducible nodes"),

            // Reducibles
            CfgNodeDescriptor::Block(cfg_block) => cfg_block.reduce(self),
        };

        // Step 6: Connect all the predecessors to `s`
        for pred in &predecessors {
            self.add_flow_edge(*pred, start_id);
        }

        // Step 7: Connect `e` to all the successors
        for succ in &successors {
            self.add_flow_edge(end_id, *succ);
        }
    }
}

impl Cfg {
    pub fn accept_block(&mut self, block: &Block) {
        let start = self.add_start_node();
        let end = self.add_end_node();
        let block = self.add_block_node(block);

        self.add_flow_edge(start, block);
        self.add_flow_edge(block, end);

        while let Some(reduction_candidate) = self.reduction_queue.pop_front() {
            self.reduce(reduction_candidate);
        }
    }
}

#[cfg(test)]
mod control_flow_tests {
    use super::*;
    use crate::detect::test_utils::load_solidity_source_unit;
    use serial_test::serial;

    #[test]
    #[serial]
    fn simple_program_function1() {
        /*

        First example
        --------------
        Consider
        ../tests/contract-playground/src/control_flow/SimpleProgram.sol
        SimpleProgram : function1

        Deconstruct the function step by step until we have a graph with only
        Every function has a body Block so we start with the following graph and reduce it to primitives

        Step 1:

            Let 'a be the ID node the CfgNode(Block b)

            reduction_queue : [ 'a ]

            Sn(Block) -> CfgNode(Block b) 'a -> En(Block)

            Short form:
            Sn -> CfgStartNode
            En -> CfgEndNode

        Step 2:

            reduction_queue: [ ]

            Sn ->
                Sn -> CfgNode(VariableDeclarationStatement v) -> En ->
                Sn -> CfgNode(ExpressionStatement e) -> En ->
                Sn -> CfgNode(ExpressionStatement e) -> En ->
            En

        */

        let context = load_solidity_source_unit(
            "../tests/contract-playground/src/control_flow/SimpleProgram.sol",
        );
        let contract = context.find_contract_by_name("SimpleProgram");
        let function = contract.find_function_by_name("function1");
        let mut cfg = Cfg::new();

        cfg.accept_block(function.body.as_ref().expect("function1 not to be defined"));

        assert!(!cfg.nodes.is_empty());
        assert_eq!(cfg.total_nodes(), 3);
    }
}
