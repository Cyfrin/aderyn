use crate::ast::*;
use std::fmt::Display;

impl Display for Statement {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Statement::VariableDeclarationStatement(stmt) => stmt.fmt(f),
            Statement::IfStatement(stmt) => stmt.fmt(f),
            Statement::ForStatement(stmt) => stmt.fmt(f),
            Statement::WhileStatement(stmt) => stmt.fmt(f),
            Statement::EmitStatement(stmt) => stmt.fmt(f),
            Statement::TryStatement(stmt) => stmt.fmt(f),
            Statement::RevertStatement(stmt) => stmt.fmt(f),
            Statement::UncheckedBlock(stmt) => stmt.fmt(f),
            Statement::Return(stmt) => stmt.fmt(f),
            Statement::ExpressionStatement(stmt) => stmt.fmt(f),
            Statement::InlineAssembly(..) => {
                f.write_str("assembly { /* WARNING: not implemented */ }")
            }
            _ => f.write_str("unrecognized!"),
        }
    }
}

impl Display for ExpressionStatement {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_fmt(format_args!("{}", self.expression))
    }
}

impl Display for VariableDeclarationStatement {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        if self.declarations.len() == 1 {
            if let Some(declaration) = self.declarations[0].as_ref() {
                f.write_fmt(format_args!("{declaration}"))?;
            } else {
                f.write_str("()")?;
            }
        } else {
            f.write_str("(")?;

            for (i, declaration) in self.declarations.iter().enumerate() {
                if i > 0 {
                    f.write_str(", ")?;
                }

                if let Some(declaration) = declaration {
                    f.write_fmt(format_args!("{declaration}"))?;
                }
            }

            f.write_str(")")?;
        }

        if let Some(initial_value) = self.initial_value.as_ref() {
            f.write_fmt(format_args!(" = {initial_value}"))?;
        }

        Ok(())
    }
}

impl Display for BlockOrStatement {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            BlockOrStatement::Block(block) => block.fmt(f),
            BlockOrStatement::Statement(statement) => statement.fmt(f),
        }
    }
}

impl Display for IfStatement {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_fmt(format_args!("if ({}) {}", self.condition, self.true_body))?;

        if let Some(false_body) = self.false_body.as_ref() {
            f.write_fmt(format_args!("\nelse {false_body}"))?;
        }

        Ok(())
    }
}

impl Display for ForStatement {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str("for (")?;

        if let Some(initialization_expression) = self.initialization_expression.as_ref() {
            f.write_fmt(format_args!("{:?}", initialization_expression))?;
        }

        f.write_str("; ")?;

        if let Some(condition) = self.condition.as_ref() {
            f.write_fmt(format_args!("{condition}"))?;
        }

        f.write_str("; ")?;

        if let Some(loop_expression) = self.loop_expression.as_ref() {
            f.write_fmt(format_args!("{loop_expression}"))?;
        }

        f.write_fmt(format_args!(") {}", self.body))
    }
}

impl Display for WhileStatement {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_fmt(format_args!("while ({}) {}", self.condition, self.body))
    }
}

impl Display for DoWhileStatement {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_fmt(format_args!("do {} while({});", self.body, self.condition))
    }
}

impl Display for EmitStatement {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_fmt(format_args!("emit {}", self.event_call))
    }
}

impl Display for TryStatement {
    fn fmt(&self, _: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        unimplemented!()
    }
}

impl Display for RevertStatement {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_fmt(format_args!("revert {}", self.error_call))
    }
}

impl Display for TryCatchClause {
    fn fmt(&self, _: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        unimplemented!()
    }
}

impl Display for Return {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str("return")?;

        if let Some(expression) = self.expression.as_ref() {
            f.write_fmt(format_args!(" {expression}"))?;
        }

        Ok(())
    }
}

impl Display for Break {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str("break;")
    }
}

impl Display for Continue {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str("continue;")
    }
}
