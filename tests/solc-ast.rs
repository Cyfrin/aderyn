#[cfg(test)]
mod abi_encode_packed_tests {
    use eyre::Result;
    use solc_ast::{ast::*, visitor::ast_visitor::*};

    fn read_abi_encode_packed() -> Result<SourceUnit> {
        Ok(serde_json::from_reader(std::io::BufReader::new(
            std::fs::File::open("tests/ast-json/AbiEncodePacked.json")?,
        ))?)
    }

    #[test]
    fn deserialize_abi_encode_packed() -> Result<()> {
        let source_unit = read_abi_encode_packed()?;
        assert_eq!(source_unit.absolute_path, Some("src/AbiEncodePacked.sol".into()));
        Ok(())
    }

    #[derive(Default, Debug)]
    struct AbiEncodePackedCollisionCollector {
        nodes: Vec<MemberAccess>,
    }

    impl ASTConstVisitor for AbiEncodePackedCollisionCollector {
        fn end_visit_member_access(&mut self, node: &MemberAccess) -> Result<()> {
            if node.member_name == "encodePacked" {
                // TODO This captures all string and bytes values in encodePacked
                // but if they're static (ie, defined in storage but never changed),
                // then they shouldn't be caught. Figure a way of doing that by also
                // collecting the variable declarations and checking if the variable
                // is static.
                for arg in node.argument_types.as_ref().unwrap() {
                    if arg.type_string.as_ref().unwrap().contains("string") || arg.type_string.as_ref().unwrap().contains("bytes") {
                        self.nodes.push(node.clone());
                        break;
                    }
                }
            }
            Ok(())
        }
    }

    #[test]
    fn encode_packed() -> Result<()> {
        let source_unit = read_abi_encode_packed()?;
        let mut abi_encode_packed_collision_collector = AbiEncodePackedCollisionCollector::default();
        source_unit.accept(&mut abi_encode_packed_collision_collector)?;
        println!("{:?}", abi_encode_packed_collision_collector.nodes[0]);
        Ok(())
    }
}

#[cfg(test)]
mod counter_tests {
    use eyre::Result;
    use solc_ast::{ast::*, visitor::ast_visitor::*};

    fn read_counter() -> Result<SourceUnit> {
        Ok(serde_json::from_reader(std::io::BufReader::new(
            std::fs::File::open("tests/ast-json/Counter.json")?,
        ))?)
    }

    #[test]
    fn deserialize_counter() -> Result<()> {
        let source_unit = read_counter()?;
        assert_eq!(source_unit.absolute_path, Some("Counter.sol".into()));
        Ok(())
    }

    #[derive(Default, Debug)]
    struct FunctionDefinitionCollector {
        names: Vec<String>,
    }

    impl ASTConstVisitor for FunctionDefinitionCollector {
        fn end_visit_function_definition(&mut self, node: &FunctionDefinition) -> Result<()> {
            self.names.push(node.name.clone());
            Ok(())
        }
    }

    #[test]
    fn functions() -> Result<()> {
        let source_unit = read_counter()?;
        let mut function_definition_collector = FunctionDefinitionCollector::default();
        source_unit.accept(&mut function_definition_collector)?;
        assert_eq!(
            function_definition_collector.names,
            vec![String::from("setNumber"), String::from("increment")]
        );
        Ok(())
    }

    #[derive(Default, Debug)]
    struct VariableNameCollector {
        names: Vec<String>,
    }

    impl ASTConstVisitor for VariableNameCollector {
        fn end_visit_variable_declaration(&mut self, node: &VariableDeclaration) -> Result<()> {
            self.names.push(node.name.clone());
            Ok(())
        }
    }

    #[test]
    fn variables() -> Result<()> {
        let source_unit = read_counter()?;
        let mut variable_name_collector = VariableNameCollector::default();
        source_unit.accept(&mut variable_name_collector)?;
        assert_eq!(
            variable_name_collector.names,
            vec![String::from("number"), String::from("newNumber")]
        );
        Ok(())
    }
}
