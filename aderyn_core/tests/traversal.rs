mod common;

use aderyn_core::detect::{detector::IssueDetector, test_utils::load_solidity_source_unit};
use common::*;

#[test]
fn test_immediate_parent_demo() {
    let context = load_solidity_source_unit(
        "../tests/contract-playground/src/parent_chain/ParentChainContract.sol",
    );

    let mut detector = ImmediateParentDemonstrator::default();
    let found = detector.detect(&context).unwrap();
    assert!(found);

    println!("Total number of instances: {:?}", detector.instances().len());
    assert!(detector.instances().len() == 3);
}

#[test]
fn test_immediate_child_demo() {
    let context = load_solidity_source_unit(
        "../tests/contract-playground/src/parent_chain/ParentChainContract.sol",
    );

    let mut detector = ImmediateChildrenDemonstrator::default();
    let found = detector.detect(&context).unwrap();
    assert!(found);

    println!("Total number of instances: {:?}", detector.instances().len());
    assert!(detector.instances().len() == 1);
}

#[test]
fn test_closest_ancestor() {
    let context = load_solidity_source_unit(
        "../tests/contract-playground/src/parent_chain/ParentChainContract.sol",
    );

    let mut detector = ClosestAncestorDemonstrator::default();
    let found = detector.detect(&context).unwrap();
    assert!(found);

    println!("Total number of instances: {:?}", detector.instances().len());
    assert!(detector.instances().len() == 4);
}

#[test]
fn test_ancestral_line_demo() {
    let context = load_solidity_source_unit(
        "../tests/contract-playground/src/parent_chain/ParentChainContract.sol",
    );

    let mut detector = AncestralLineDemonstrator::default();
    let found = detector.detect(&context).unwrap();
    assert!(found);

    println!("Total number of instances: {:?}", detector.instances().len());
    assert!(detector.instances().len() == 4);
}

#[test]
fn test_new_ast_nodes() {
    let context = load_solidity_source_unit("../tests/adhoc-sol-files/DemoASTNodes.sol");

    let mut detector = NewASTNodesDemonstrator::default();
    let _ = detector.detect(&context).unwrap();

    let instances = detector.instances();
    //println!("{:?}", instances);

    assert!(instances.len() == 4);
}

#[test]
fn transient_can_compile() {
    load_solidity_source_unit("../tests/contract-playground/src/TransientKeyword.sol");
}

#[test]
fn test_peek_over() {
    let context =
        load_solidity_source_unit("../tests/contract-playground/src/StorageConditionals.sol");
    let mut detector = PeekOverDemonstrator::default();
    let _ = detector.detect(&context).unwrap();

    let instances = detector.instances();
    assert!(instances.len() == 2);
}

#[test]
fn test_siblings() {
    let context =
        load_solidity_source_unit("../tests/contract-playground/src/StorageConditionals.sol");

    let mut detector = SiblingDemonstrator::default();
    let _ = detector.detect(&context).unwrap();
    assert_eq!(detector.instances().len(), 1);
}
