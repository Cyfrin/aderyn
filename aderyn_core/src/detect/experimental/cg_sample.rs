#![allow(clippy::collapsible_match)]

#[cfg(test)]
mod callgraph_tests {

    #[test]
    fn test_callgraph_is_not_none() {
        let context = crate::detect::test_utils::load_solidity_source_unit_with_callgraphs(
            "../tests/contract-playground/src/parent_chain/ParentChainContract.sol",
        );
        assert!(context.forward_callgraph.is_some());
        assert!(context.reverse_callgraph.is_some());
    }
}
