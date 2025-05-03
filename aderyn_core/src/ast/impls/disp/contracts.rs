use crate::ast::*;
use std::fmt::Display;

impl Display for ContractKind {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_fmt(format_args!("{}", format!("{self:?}").to_lowercase()))
    }
}

impl Display for ContractDefinitionNode {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ContractDefinitionNode::UsingForDirective(using_for_directive) => {
                using_for_directive.fmt(f)
            }
            ContractDefinitionNode::StructDefinition(struct_definition) => struct_definition.fmt(f),
            ContractDefinitionNode::EnumDefinition(enum_definition) => enum_definition.fmt(f),
            ContractDefinitionNode::VariableDeclaration(variable_declaration) => {
                variable_declaration.fmt(f)
            }
            ContractDefinitionNode::EventDefinition(event_definition) => event_definition.fmt(f),
            ContractDefinitionNode::FunctionDefinition(function_definition) => {
                function_definition.fmt(f)
            }
            ContractDefinitionNode::ModifierDefinition(modifier_definition) => {
                modifier_definition.fmt(f)
            }
            ContractDefinitionNode::ErrorDefinition(error_definition) => error_definition.fmt(f),
            ContractDefinitionNode::UserDefinedValueTypeDefinition(
                user_defined_value_type_definition,
            ) => user_defined_value_type_definition.fmt(f),
        }
    }
}

impl Display for InheritanceSpecifier {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_fmt(format_args!("{:?}", self.base_name))?;

        if let Some(arguments) = self.arguments.as_ref() {
            f.write_str("(")?;

            for (i, argument) in arguments.iter().enumerate() {
                f.write_fmt(format_args!(
                    "{}{}",
                    match i {
                        0 => "",
                        _ => ", ",
                    },
                    argument,
                ))?;
            }

            f.write_str(")")?;
        }

        Ok(())
    }
}

impl Display for ContractDefinition {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        if self.is_abstract {
            f.write_str("abstract ")?;
        }

        f.write_fmt(format_args!("{} {}", self.kind, self.name))?;

        for (i, base_contract) in self.base_contracts.iter().enumerate() {
            f.write_fmt(format_args!(
                "{}{}",
                match i {
                    0 => " is ",
                    _ => ", ",
                },
                base_contract
            ))?;
        }

        f.write_str(" {\n")?;

        for node in self.nodes.iter() {
            f.write_fmt(format_args!(
                "\t{}{}\n",
                node,
                match node {
                    ContractDefinitionNode::UsingForDirective(_)
                    | ContractDefinitionNode::EventDefinition(_)
                    | ContractDefinitionNode::ErrorDefinition(_)
                    | ContractDefinitionNode::VariableDeclaration(_)
                    | ContractDefinitionNode::UserDefinedValueTypeDefinition(_) => ";",

                    ContractDefinitionNode::StructDefinition(_)
                    | ContractDefinitionNode::EnumDefinition(_)
                    | ContractDefinitionNode::FunctionDefinition(_)
                    | ContractDefinitionNode::ModifierDefinition(_) => "",
                }
            ))?;
        }

        f.write_str("}")?;

        Ok(())
    }
}
