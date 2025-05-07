//! This module helps us detect whether a given AST Node has any external calls inside of it

use super::ExtractMemberAccesses;
use crate::context::workspace::ASTNode;

pub fn is_extcallish(ast_node: ASTNode) -> bool {
    // This is so we can skip the FunctionCallOptions layer which solidity compiler inserts
    // when there are options passed to function calls
    for member_access in ExtractMemberAccesses::from(&ast_node).extracted {
        // address(..).call("...") pattern
        let is_call = member_access.member_name == "call";
        if is_call {
            return true;
        }

        // payable(address(..)).transfer(100)
        // payable(address(..)).send(100)
        // address.sendValue(..) (from openzeppelin)
        if member_access.member_name == "transfer"
            || member_access.member_name == "send"
            || member_access.member_name == "sendValue"
        {
            if let Some(type_description) = member_access.expression.type_descriptions() {
                if type_description
                    .type_string
                    .as_ref()
                    .is_some_and(|type_string| type_string.starts_with("address"))
                {
                    return true;
                }
            }
        }

        // Any external call
        if member_access
            .type_descriptions
            .type_identifier
            .is_some_and(|type_identifier| type_identifier.contains("function_external"))
        {
            return true;
        }
    }

    false
}

#[cfg(test)]
mod external_calls_detector {

    use crate::{
        ast::*, context::browser::ExtractFunctionCalls,
        detect::test_utils::load_solidity_source_unit,
    };

    impl FunctionDefinition {
        pub fn makes_external_calls(&self) -> bool {
            let func_calls = ExtractFunctionCalls::from(self).extracted;
            func_calls.iter().any(|f| f.is_extcallish())
        }
    }

    #[test]

    fn test_direct_call_on_address() {
        let context =
            load_solidity_source_unit("../tests/contract-playground/src/ExternalCalls.sol");

        let childex = context.find_contract_by_name("ChildEx");

        let ext1 = childex.find_function_by_name("ext1");
        let ext2 = childex.find_function_by_name("ext2");
        let ext3 = childex.find_function_by_name("ext3");
        let ext4 = childex.find_function_by_name("ext4");
        let ext5 = childex.find_function_by_name("ext5");
        let ext6 = childex.find_function_by_name("ext6");
        let ext7 = childex.find_function_by_name("ext7");
        let ext8 = childex.find_function_by_name("ext8");
        let ext9 = childex.find_function_by_name("ext9");

        assert!(ext1.makes_external_calls());
        assert!(ext2.makes_external_calls());
        assert!(ext3.makes_external_calls());
        assert!(ext4.makes_external_calls());
        assert!(ext5.makes_external_calls());
        assert!(ext6.makes_external_calls());
        assert!(ext7.makes_external_calls());
        assert!(ext8.makes_external_calls());
        assert!(ext9.makes_external_calls());

        let notext1 = childex.find_function_by_name("notExt1");
        let notext2 = childex.find_function_by_name("notExt2");

        assert!(!notext1.makes_external_calls());
        assert!(!notext2.makes_external_calls());
    }
}
