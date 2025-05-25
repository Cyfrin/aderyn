use std::collections::HashSet;

use super::{voids::CfgEndNode, ASTNode, Cfg, CfgNode, CfgNodeDescriptor, CfgNodeId, Statement};
use crate::context::workspace::WorkspaceContext;

impl Cfg {
    pub fn add_statement_node(&mut self, stmt: &Statement) -> CfgNodeId {
        match stmt {
            Statement::TryStatement(n) => self.add_try_statement(n),
            Statement::UncheckedBlock(n) => self.add_unchecked_block_node(n),
            Statement::DoWhileStatement(n) => self.add_do_while_statement(n),
            Statement::ForStatement(n) => self.add_for_statement(n),
            Statement::EmitStatement(n) => self.add_emit_statement(n),
            Statement::WhileStatement(n) => self.add_while_statement(n),
            Statement::IfStatement(n) => self.add_if_statement(n),
            Statement::PlaceholderStatement(n) => self.add_placeholder_statement(n),
            Statement::RevertStatement(n) => self.add_revert_statement(n),
            Statement::Return(n) => self.add_return_statement(n),
            Statement::InlineAssembly(n) => self.add_inline_assembly_statement(n),
            Statement::Continue(n) => self.add_continue_statement(n),
            Statement::Break(n) => self.add_break_statement(n),
            Statement::Block(n) => self.add_block_node(n),
            Statement::VariableDeclarationStatement(n) => {
                self.add_variable_declaration_statement(n)
            }
            Statement::ExpressionStatement(n) => self.add_expression_statement(n),
        }
    }
}

pub(crate) fn discover_jump_sources(
    cfg: &Cfg,
    curr_node: CfgNodeId,
    visited: &mut HashSet<CfgNodeId>,
    continue_statements: &mut Vec<CfgNodeId>,
    break_statements: &mut Vec<CfgNodeId>,
    return_statements: &mut Vec<CfgNodeId>,
) {
    if visited.contains(&curr_node) {
        return;
    }

    visited.insert(curr_node);

    let curr_node = cfg.nodes.get(&curr_node).expect("Invalid Cfg Node ID passed in");

    match &curr_node.nd {
        CfgNodeDescriptor::Continue(_) => {
            continue_statements.push(curr_node.id);
        }
        CfgNodeDescriptor::Break(_) => {
            break_statements.push(curr_node.id);
        }
        CfgNodeDescriptor::Return(_) => {
            return_statements.push(curr_node.id);
        }
        _ => {}
    };

    let children = cfg.raw_successors(curr_node.id);

    for child in children {
        discover_jump_sources(
            cfg,
            child,
            visited,
            continue_statements,
            break_statements,
            return_statements,
        );
    }
}

pub(crate) fn find_jump_dest(
    cfg: &Cfg,
    curr_node: CfgNodeId,
    visited: &mut HashSet<CfgNodeId>,
) -> Option<JumpDestination> {
    if visited.contains(&curr_node) {
        return None;
    }

    visited.insert(curr_node);

    // Ensure that `curr_node` is continue, break or return. It may work otherwise but that is not
    // intended behavior
    let curr_node = cfg.nodes.get(&curr_node).expect("Invalid Cfg Node ID passed in");

    if let CfgNodeDescriptor::End(end) = &curr_node.nd {
        match end.as_ref() {
            CfgEndNode::EndWhileCond => {
                return Some(JumpDestination::While(
                    *cfg.cond_start_pairs.get(&curr_node.id).expect("Unfilled!"),
                ));
            }
            CfgEndNode::EndDoWhileCond => {
                return Some(JumpDestination::DoWhile(
                    *cfg.cond_start_pairs.get(&curr_node.id).expect("Unfilled!"),
                ));
            }
            CfgEndNode::EndForLoopExp => {
                return Some(JumpDestination::For(
                    *cfg.loop_expr_start.get(&curr_node.id).expect("Unfilled!"),
                ));
            }
            _ => {}
        };
    }

    let children: Vec<CfgNodeId> = match &curr_node.nd {
        CfgNodeDescriptor::Start(_) => {
            vec![*cfg.start_end_pairs.get(&curr_node.id).expect("Unfilled!")]
        }
        _ => cfg.raw_successors(curr_node.id),
    };

    for child in children {
        if let Some(jump_dest) = find_jump_dest(cfg, child, visited) {
            return Some(jump_dest);
        }
    }

    None
}

#[derive(Clone, Copy, Debug)]
pub enum JumpDestination {
    /// If there is no parent loop, i.e the parent loop is outside the scope covered by
    /// `start_node`, then the break and continue statements should flow to the default
    /// site which is the `end_node`
    DefaultSite,

    For(CfgNodeId),
    DoWhile(CfgNodeId),
    While(CfgNodeId),
}

impl Default for JumpDestination {
    fn default() -> Self {
        Self::DefaultSite
    }
}

pub enum Calibration {
    ContinueShouldFlowTo(CfgNodeId, JumpDestination),
    BreakShouldFlowTo(CfgNodeId, JumpDestination),
    ReturnShouldFlowToEndNode(CfgNodeId),
}

impl Cfg {
    pub(crate) fn calibrate(
        &mut self,
        proposed_calibrations: Vec<Calibration>,
        end_node: CfgNodeId,
    ) {
        for proposed_calibration in proposed_calibrations {
            let (target, calibrated_dest) = match proposed_calibration {
                Calibration::ContinueShouldFlowTo(c, JumpDestination::For(for_node)) => {
                    (c, self.find_loop_expression_node(for_node))
                }
                Calibration::ContinueShouldFlowTo(c, JumpDestination::While(while_node)) => {
                    (c, self.find_condition_node(while_node))
                }
                Calibration::ContinueShouldFlowTo(c, JumpDestination::DoWhile(do_while_node)) => {
                    (c, self.find_condition_node(do_while_node))
                }
                Calibration::ContinueShouldFlowTo(c, JumpDestination::DefaultSite) => (c, end_node),
                Calibration::BreakShouldFlowTo(b, JumpDestination::For(for_node)) => {
                    (b, self.find_ending_counter_part(for_node))
                }
                Calibration::BreakShouldFlowTo(b, JumpDestination::While(while_node)) => {
                    (b, self.find_ending_counter_part(while_node))
                }
                Calibration::BreakShouldFlowTo(b, JumpDestination::DoWhile(do_while_node)) => {
                    (b, self.find_ending_counter_part(do_while_node))
                }
                Calibration::BreakShouldFlowTo(b, JumpDestination::DefaultSite) => (b, end_node),
                Calibration::ReturnShouldFlowToEndNode(r) => (r, end_node),
            };
            self.reset_raw_successors(target, calibrated_dest);
        }
    }
}

impl CfgNodeDescriptor {
    pub fn reflect<'a>(&self, context: &'a WorkspaceContext) -> Option<&'a ASTNode> {
        let ast_node_id = match self {
            CfgNodeDescriptor::Start(_) => None,
            CfgNodeDescriptor::End(_) => None,
            CfgNodeDescriptor::VariableDeclarationStatement(n) => {
                Some(n.variable_declaration_statement)
            }
            CfgNodeDescriptor::ExpressionStatement(n) => Some(n.expression_statement),
            CfgNodeDescriptor::PlaceholderStatement(n) => Some(n.placeholder_statement),
            CfgNodeDescriptor::Break(n) => Some(n.break_statement),
            CfgNodeDescriptor::Continue(n) => Some(n.continue_statement),
            CfgNodeDescriptor::Return(n) => Some(n.return_statement),
            CfgNodeDescriptor::EmitStatement(n) => Some(n.emit_statement),
            CfgNodeDescriptor::RevertStatement(n) => Some(n.revert_statement),
            CfgNodeDescriptor::InlineAssembly(n) => Some(n.inline_assembly_statement),
            CfgNodeDescriptor::IfStatementCondition(n) => n.if_stmt_condition,
            CfgNodeDescriptor::WhileStatementCondition(n) => n.while_stmt_condition,
            CfgNodeDescriptor::ForStatementCondition(n) => n.for_stmt_condition,
            CfgNodeDescriptor::DoWhileStatementCondition(n) => n.do_while_stmt_condition,
            CfgNodeDescriptor::Block(n) => Some(n.block),
            CfgNodeDescriptor::UncheckedBlock(n) => Some(n.unchecked_block),
            CfgNodeDescriptor::IfStatement(n) => Some(n.if_statement),
            CfgNodeDescriptor::WhileStatement(n) => Some(n.while_statement),
            CfgNodeDescriptor::ForStatement(n) => Some(n.for_statement),
            CfgNodeDescriptor::DoWhileStatement(n) => Some(n.do_while_statement),
            CfgNodeDescriptor::TryStatement(n) => Some(n.try_statement),
        };
        context.nodes.get(&ast_node_id?)
    }
}

impl CfgNode {
    pub fn reflect<'a>(&self, context: &'a WorkspaceContext) -> Option<&'a ASTNode> {
        self.nd.reflect(context)
    }
}
