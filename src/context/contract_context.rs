use eyre::Result;

use solc_ast::{ast::*, visitor::ast_visitor::*};

#[derive(Default, Debug)]
pub struct ContractContext {
    pub pragma: PragmaDirective
}

impl ASTConstVisitor for ContractContext {
    fn end_visit_pragma_directive(&mut self, node: &PragmaDirective) -> Result<()> {
        self.pragma = node.clone();
        Ok(())
    }
}

#[cfg(test)]
mod contract_context_tests {
    use eyre::Result;
    use solc_ast::{ast::*, visitor::ast_visitor::*};
    use crate::context::contract_context::ContractContext;

    fn read_abi_encode_packed() -> Result<SourceUnit> {
        Ok(serde_json::from_reader(std::io::BufReader::new(
            std::fs::File::open("tests/ast-json/AbiEncodePacked.json")?,
        ))?)
    }

    #[test]
    fn pragma_directives() -> Result<()> {
        let source_unit = read_abi_encode_packed()?;
        let mut context = ContractContext::default();
        source_unit.accept(&mut context)?;
        println!("{:?}", context.pragma);
        Ok(())
    }
}