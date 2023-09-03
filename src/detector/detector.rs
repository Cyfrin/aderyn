use std::fmt::Debug;

use eyre::Result;

use solc_ast::{ast::*, visitor::ast_visitor::*};

use crate::context::contract_context::{ContractContext, ASTNode};

pub struct Detectors {
    pub context: ContractContext,
    pub encode_packed_collisions: Vec<FunctionCall>,
}

impl Detectors {
    pub fn new(context: ContractContext) -> Self {
        Self {
            context: context,
            encode_packed_collisions: Vec::new(),
        }
    }

    pub fn is_variable_length(&self, id: i64) -> bool {
        // Get the node from the context
        // If it's a StateVariable or NonStateVariable
        // Check if its of type string, bytes or array. If it is, return true.
        // Otherwise, return false.
        if let Some(node) = self.context.get(id) {
            match node {
                ASTNode::StateVariable(variable_declaration) => {
                    if let Some(type_name) = &variable_declaration.type_name {
                        if let TypeName::ElementaryTypeName(elementary_type_name) = type_name {
                            if elementary_type_name.name == "string" || elementary_type_name.name == "bytes" {
                                return true;
                            }
                        }
                        if let TypeName::ArrayTypeName(_array_type_name) = type_name {
                            return true;
                        }
                    }
                },
                ASTNode::NonStateVariable(variable_declaration) => {
                    if let Some(type_name) = &variable_declaration.type_name {
                        if let TypeName::ElementaryTypeName(elementary_type_name) = type_name {
                            if elementary_type_name.name == "string" || elementary_type_name.name == "bytes" {
                                return true;
                            }
                        }
                        if let TypeName::ArrayTypeName(_array_type_name) = type_name {
                            return true;
                        }
                    }
                },
                _ => {},
            }
        }
        false
    }
}

impl ASTConstVisitor for Detectors {
    fn visit_source_unit(&mut self, node: &SourceUnit) -> Result<bool> {
        println!("SECOND PASS");
        Ok(true)
    }

    fn end_visit_function_call(&mut self, node: &FunctionCall) -> Result<()> {
        // If the expression inside the function call is a MemberAccess, and that MemberAccess
        // is a call to encodePacked, then check the arguments to the function call for more than
        // one variable length array
        if let Expression::MemberAccess(member_access) = node.expression.as_ref().clone() {
            if member_access.member_name == "encodePacked" {
                let mut variable_length_parameters = 0;
                for arg in &node.arguments {
                    if let Expression::Identifier(identifier) = arg {
                        if self.is_variable_length(identifier.referenced_declaration) {
                            variable_length_parameters += 1;
                        }
                        else {
                            variable_length_parameters = 0;
                        }
                    }
                    if variable_length_parameters == 2 {
                        self.encode_packed_collisions.push(node.clone());
                        break;
                    }
                }

            }
        }
        Ok(())  
    }
}

#[cfg(test)]
mod detector_tests {
    use eyre::Result;
    use solc_ast::{ast::*, visitor::ast_visitor::*};
    use crate::context::contract_context::ContractContext;
    use crate::detector::detector::Detectors;

    fn read_abi_encode_packed() -> Result<SourceUnit> {
        Ok(serde_json::from_reader(std::io::BufReader::new(
            // std::fs::File::open("tests/ast-json/StateVariables.ast.json")?,
            std::fs::File::open("tests/ast-json/AbiEncodePacked.json")?,
        ))?)
    }

    #[test]
    fn test_encode_packed_detector() -> Result<()> {
        let source_unit = read_abi_encode_packed()?;
        let mut context = ContractContext::default();
        source_unit.accept(&mut context)?;
        let mut detectors = Detectors::new(context);
        source_unit.accept(&mut detectors)?;
        detectors.encode_packed_collisions.iter().for_each(|node| {
            println!("Node ID: {}, Position: {}", node.id, node.src);
        });
        Ok(())
    }
}