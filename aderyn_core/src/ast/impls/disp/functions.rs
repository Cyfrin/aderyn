use crate::ast::*;
use std::fmt::Display;

impl Display for FunctionKind {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_fmt(format_args!("{}", format!("{self:?}").to_lowercase()))
    }
}

impl Display for ParameterList {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str("(")?;

        for (i, parameter) in self.parameters.iter().enumerate() {
            if i > 0 {
                f.write_str(", ")?;
            }

            f.write_fmt(format_args!("{parameter}"))?;
        }

        f.write_str(")")
    }
}

impl Display for OverrideSpecifier {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str("override")?;

        if !self.overrides.is_empty() {
            f.write_str("(")?;

            for (i, identifier_path) in self.overrides.iter().enumerate() {
                if i > 0 {
                    f.write_str(", ")?;
                }

                f.write_fmt(format_args!("{:?}", identifier_path))?;
            }

            f.write_str(")")?;
        }

        Ok(())
    }
}

impl Display for FunctionDefinition {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_fmt(format_args!("{}", self.kind()))?;

        if !self.name.is_empty() {
            f.write_fmt(format_args!(" {}", self.name))?;
        }

        f.write_fmt(format_args!("{} {}", self.parameters, self.visibility))?;

        if let Some(state_mutability) = &self.state_mutability {
            if *state_mutability != StateMutability::NonPayable {
                f.write_fmt(format_args!(" {}", state_mutability))?;
            }
        }

        if self.is_virtual {
            f.write_str(" virtual")?;
        }

        if let Some(overrides) = self.overrides.as_ref() {
            f.write_fmt(format_args!(" {overrides}"))?;
        }

        for modifier in self.modifiers.iter() {
            f.write_fmt(format_args!(" {modifier}"))?;
        }

        if !self.return_parameters.parameters.is_empty() {
            f.write_fmt(format_args!(" returns {}", self.return_parameters))?;
        }

        match self.body.as_ref() {
            Some(body) => f.write_fmt(format_args!(" {body}")),
            None => f.write_str(";"),
        }
    }
}
