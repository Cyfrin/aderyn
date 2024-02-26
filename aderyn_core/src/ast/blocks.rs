use super::*;
use crate::visitor::ast_visitor::*;
use eyre::Result;
use serde::{Deserialize, Serialize};
use std::fmt::Display;

#[derive(Clone, Debug, Deserialize, Eq, Serialize, PartialEq, Hash)]
#[serde(rename_all = "camelCase")]
pub struct Block {
    pub statements: Vec<Statement>,
    pub src: String,
    pub id: NodeID,
}

impl Node for Block {
    fn accept(&self, visitor: &mut impl ASTConstVisitor) -> Result<()> {
        if visitor.visit_block(self)? {
            list_accept(&self.statements, visitor)?;
        }
        self.accept_metadata(visitor)?;
        visitor.end_visit_block(self)
    }
}

impl Block {
    fn accept_metadata(&self, visitor: &mut impl ASTConstVisitor) -> Result<()> {
        let children_ids = self
            .statements
            .iter()
            .map(|x| match x {
                Statement::VariableDeclarationStatement(n) => Some(n.id),
                Statement::IfStatement(n) => Some(n.id),
                Statement::ForStatement(n) => Some(n.id),
                Statement::WhileStatement(n) => Some(n.id),
                Statement::InlineAssembly(n) => Some(n.id),
                Statement::UncheckedBlock(n) => Some(n.id),
                Statement::UnhandledStatement {
                    node_type: _,
                    src: _,
                    id,
                } => *id,
                Statement::Return(n) => Some(n.id),
                Statement::EmitStatement(_)
                | Statement::TryStatement(_)
                | Statement::RevertStatement(_)
                | Statement::ExpressionStatement(_) => None,
            })
            .collect::<Vec<_>>();
        visitor.visit_immediate_children(self.id, children_ids.into_iter().flatten().collect())
    }
}

impl Display for Block {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str("{\n")?;

        for statement in self.statements.iter() {
            f.write_fmt(format_args!("\t{statement};\n"))?;
        }

        f.write_str("}")
    }
}
