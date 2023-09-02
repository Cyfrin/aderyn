use std::fmt::Debug;

use eyre::Result;

use solc_ast::{ast::*, visitor::ast_visitor::*};

use crate::context::contract_context::ContractContext;

pub struct Detectors {
    pub context: ContractContext,
}

impl Detectors {
    pub fn new(context: ContractContext) -> Self {
        Self {
            context: context,
        }
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
                for arg in &node.arguments {
                    if let Expression::Identifier(identifier) = arg {
                        // TODO: Have a counter for the number of variable length parameters
                        // and if it's greater than 1, then add the function call to the list of encodePacked Collisions.
                        // Variable length parameters in this case are:
                        // - any Variable Declaration,
                        // - of type string, bytes or array,
                        // - and not in constant_variables or immutable_variables.
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
        Ok(())
    }
}