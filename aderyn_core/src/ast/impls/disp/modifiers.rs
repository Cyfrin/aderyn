use crate::ast::*;
use std::fmt::Display;

impl Display for ModifierDefinition {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str("modifier")?;

        if !self.name.is_empty() {
            f.write_fmt(format_args!(" {}", self.name))?;
        }

        f.write_fmt(format_args!("{}", self.parameters))?;

        if self.visibility != Visibility::Internal {
            f.write_fmt(format_args!("{} {}", self.parameters, self.visibility))?;
        }

        if self.is_virtual {
            f.write_fmt(format_args!(" virtual"))?;
        }

        if let Some(overrides) = self.overrides.as_ref() {
            f.write_fmt(format_args!(" {overrides}"))?;
        }

        if let Some(body) = self.body.as_ref() {
            f.write_fmt(format_args!(" {}", body))?;
        }

        Ok(())
    }
}

impl Display for ModifierInvocation {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_fmt(format_args!("{:?}", self.modifier_name))?;

        if let Some(arguments) = self.arguments.as_ref() {
            f.write_str("(")?;

            for (i, argument) in arguments.iter().enumerate() {
                if i > 0 {
                    f.write_str(", ")?;
                }

                f.write_fmt(format_args!("{argument}"))?;
            }

            f.write_str(")")?;
        }

        Ok(())
    }
}
