use crate::ast::*;
use std::fmt::{Display, Write};

impl Display for Expression {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Expression::Literal(expr) => expr.fmt(f)?,
            Expression::Identifier(expr) => expr.fmt(f)?,
            Expression::UnaryOperation(expr) => expr.fmt(f)?,
            Expression::BinaryOperation(expr) => expr.fmt(f)?,
            Expression::Conditional(expr) => expr.fmt(f)?,
            Expression::Assignment(expr) => expr.fmt(f)?,
            Expression::FunctionCall(expr) => expr.fmt(f)?,
            Expression::FunctionCallOptions(expr) => expr.fmt(f)?,
            Expression::IndexAccess(expr) => expr.fmt(f)?,
            Expression::IndexRangeAccess(expr) => expr.fmt(f)?,
            Expression::MemberAccess(expr) => expr.fmt(f)?,
            Expression::ElementaryTypeNameExpression(expr) => expr.fmt(f)?,
            Expression::TupleExpression(expr) => expr.fmt(f)?,
            Expression::NewExpression(expr) => expr.fmt(f)?,
        }

        Ok(())
    }
}

impl Display for UnaryOperation {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_fmt(format_args!("{}{}", self.sub_expression, self.operator.as_str()))
    }
}

impl Display for BinaryOperation {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_fmt(format_args!(
            "{} {} {}",
            self.left_expression, self.operator, self.right_expression
        ))
    }
}

impl Display for Conditional {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_fmt(format_args!(
            "{} ? {} : {}",
            self.condition, self.true_expression, self.false_expression
        ))
    }
}

impl Display for Assignment {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_fmt(format_args!(
            "{} {} {}",
            self.left_hand_side, self.operator, self.right_hand_side
        ))
    }
}

impl Display for FunctionCall {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_fmt(format_args!("{}", self.expression))?;
        f.write_str("(")?;

        for (i, argument) in self.arguments.iter().enumerate() {
            if i > 0 {
                f.write_str(", ")?;
            }

            f.write_fmt(format_args!("{argument}"))?;
        }

        f.write_str(")")
    }
}

impl Display for FunctionCallOptions {
    #[allow(clippy::print_in_format_impl)]
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let option_count = self.options.len();

        if self.names.len() != option_count {
            eprintln!("ERROR: invalid FunctionCallOptions: {:?}, {:?}", self.names, self.options);

            return Err(std::fmt::Error);
        }

        f.write_fmt(format_args!("{}", self.expression))?;

        f.write_char('{')?;

        for i in 0..option_count {
            if i > 0 {
                f.write_str(", ")?;
            }

            f.write_fmt(format_args!("{}: {}", self.names[i], self.options[i]))?;
        }

        f.write_char('}')?;

        if let Some(arguments) = self.arguments.as_ref() {
            f.write_char('(')?;

            for (i, argument) in arguments.iter().enumerate() {
                if i > 0 {
                    f.write_str(", ")?;
                }

                f.write_fmt(format_args!("{argument}"))?;
            }

            f.write_char(')')?;
        }

        Ok(())
    }
}

impl Display for IndexAccess {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        if let Some(index_expression) = &self.index_expression {
            f.write_fmt(format_args!("{}[{}]", self.base_expression, index_expression))
        } else {
            f.write_fmt(format_args!("{}[]", self.base_expression))
        }
    }
}

impl Display for IndexRangeAccess {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_fmt(format_args!("{}[", self.base_expression))?;

        if let Some(start_expression) = self.start_expression.as_ref() {
            f.write_fmt(format_args!("{start_expression}"))?;
        }

        f.write_str(":")?;

        if let Some(end_expression) = self.end_expression.as_ref() {
            f.write_fmt(format_args!("{end_expression}"))?;
        }

        f.write_str("]")
    }
}

impl Display for MemberAccess {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_fmt(format_args!("{}.{}", self.expression, self.member_name))
    }
}

impl Display for ElementaryTypeNameExpression {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_fmt(format_args!("{}", self.type_name))
    }
}

impl Display for TupleExpression {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str("(")?;

        for (i, component) in self.components.iter().enumerate() {
            if i > 0 {
                f.write_str(", ")?;
            }

            if let Some(component) = component {
                f.write_fmt(format_args!("{component}"))?;
            }
        }

        f.write_str(")")
    }
}

impl Display for NewExpression {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_fmt(format_args!("new {}", self.type_name))
    }
}
