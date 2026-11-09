use std::{
    collections::hash_map::DefaultHasher,
    hash::{Hash, Hasher},
};

use super::*;
use crate::detect::{detector::IssueDetector, low::EmptyRequireRevertDetector};

fn semantic_nodes(id: NodeID, src: &str) -> Vec<ASTNode> {
    let descriptions = TypeDescriptions {
        type_identifier: Some("t_uint256".to_string()),
        type_string: Some("uint256".to_string()),
    };
    vec![
        ASTNode::Identifier(Identifier {
            id,
            src: src.to_string(),
            argument_types: Some(vec![]),
            name: "revert".to_string(),
            overloaded_declarations: vec![],
            referenced_declaration: Some(-18),
            type_descriptions: TypeDescriptions { type_identifier: None, type_string: None },
        }),
        ASTNode::IdentifierPath(IdentifierPath {
            id: id + 1,
            src: src.to_string(),
            name: "Token".to_string(),
            referenced_declaration: 100,
        }),
        ASTNode::ElementaryTypeName(ElementaryTypeName {
            id: id + 2,
            src: src.to_string(),
            state_mutability: None,
            name: "uint256".to_string(),
            type_descriptions: descriptions.clone(),
        }),
        ASTNode::UserDefinedTypeName(UserDefinedTypeName {
            id: id + 3,
            src: src.to_string(),
            path_node: None,
            referenced_declaration: 100,
            name: Some("Token".to_string()),
            type_descriptions: descriptions,
            contract_scope: None,
        }),
    ]
}

fn hash(node: &ASTNode) -> u64 {
    let mut hasher = DefaultHasher::new();
    match node {
        ASTNode::Identifier(node) => node.hash(&mut hasher),
        ASTNode::IdentifierPath(node) => node.hash(&mut hasher),
        ASTNode::ElementaryTypeName(node) => node.hash(&mut hasher),
        ASTNode::UserDefinedTypeName(node) => node.hash(&mut hasher),
        _ => unreachable!(),
    }
    hasher.finish()
}

fn assert_equal_hashes(first: &ASTNode, second: &ASTNode) {
    assert_eq!(first, second);
    assert_eq!(hash(first), hash(second), "equal nodes must hash equally");
}

fn assert_location_independent_hash(index: usize) {
    let first = semantic_nodes(10, "0:6:0");
    for alternatives in [semantic_nodes(20, "0:6:0"), semantic_nodes(10, "10:6:0")] {
        assert_equal_hashes(&first[index], &alternatives[index]);
    }
}

#[test]
fn identifier_hash_preserves_semantic_equality() {
    assert_location_independent_hash(0);
    let ASTNode::Identifier(first) = &semantic_nodes(10, "0:6:0")[0] else { unreachable!() };
    let ASTNode::Identifier(second) = &semantic_nodes(20, "10:6:0")[0] else { unreachable!() };
    assert_eq!(Expression::Identifier(first.clone()), Expression::Identifier(second.clone()));
}

#[test]
fn identifier_path_hash_preserves_semantic_equality() {
    assert_location_independent_hash(1);
}

#[test]
fn elementary_type_hash_preserves_semantic_equality() {
    assert_location_independent_hash(2);
    let first = semantic_nodes(10, "0:6:0")[2].clone();
    let mut second = first.clone();
    let ASTNode::ElementaryTypeName(node) = &mut second else { unreachable!() };
    node.name = "uint".to_string();
    assert_equal_hashes(&first, &second);
}

#[test]
fn user_defined_type_hash_preserves_semantic_equality() {
    assert_location_independent_hash(3);
    let first = semantic_nodes(10, "0:6:0")[3].clone();
    let mut second = first.clone();
    let ASTNode::UserDefinedTypeName(node) = &mut second else { unreachable!() };
    node.name = Some("Alias".to_string());
    node.path_node = Some(IdentifierPath {
        id: 200,
        src: "20:5:0".to_string(),
        name: "Alias".to_string(),
        referenced_declaration: 100,
    });
    node.type_descriptions.type_string = Some("contract Token".to_string());
    node.contract_scope = Some("Container".to_string());
    assert_equal_hashes(&first, &second);
}

fn source_unit(id: NodeID, name: &str) -> SourceUnit {
    SourceUnit {
        id,
        src: format!("0:20:{}", id - 1),
        license: None,
        nodes: vec![],
        exported_symbols: None,
        absolute_path: Some(name.to_string()),
        source: Some("revert();\nrevert();\n".to_string()),
    }
}

#[test]
fn semantic_nodes_keep_distinct_physical_occurrences() {
    let mut context = WorkspaceContext::default();
    let mut expected = vec![];
    for (unit_id, name) in [(1, "first.sol"), (2, "second.sol")] {
        source_unit(unit_id, name).accept(&mut context).unwrap();
        for (offset, line) in [(0, 1), (10, 2)] {
            let src = format!("{offset}:6:{}", unit_id - 1);
            for node in semantic_nodes(unit_id * 100 + offset, &src) {
                node.accept(&mut context).unwrap();
                expected.push((node, name, line, format!("{offset}:6")));
            }
        }
    }
    assert_eq!(context.identifiers().len(), 4);
    assert_eq!(context.identifier_paths().len(), 4);
    assert_eq!(context.elementary_type_names().len(), 4);
    assert_eq!(context.user_defined_type_names().len(), 4);
    for (node, name, line, location) in expected {
        let source = context.get_source_unit_from_child_node(&node).unwrap();
        assert_eq!(source.absolute_path.as_deref(), Some(name));
        assert_eq!(context.get_node_sort_key(&node), (name.to_string(), line, location));
    }
    let mut detector = EmptyRequireRevertDetector::default();
    assert!(detector.detect(&context).unwrap());
    assert_eq!(
        detector.instances().into_keys().collect::<Vec<_>>(),
        vec![
            ("first.sol".to_string(), 1, "0:6".to_string()),
            ("first.sol".to_string(), 2, "10:6".to_string()),
            ("second.sol".to_string(), 1, "0:6".to_string()),
            ("second.sol".to_string(), 2, "10:6".to_string()),
        ]
    );
}

#[test]
fn sourceless_node_lookup_and_source_units_keep_their_context() {
    let mut context = WorkspaceContext::default();
    let first = source_unit(1, "first.sol");
    first.accept(&mut context).unwrap();
    let yul = YulIdentifier { name: "value".to_string(), src: "10:5:0".to_string() };
    yul.accept(&mut context).unwrap();
    let block = YulBlock { statements: vec![] };
    block.accept(&mut context).unwrap();
    source_unit(2, "second.sol").accept(&mut context).unwrap();
    assert_eq!(context.source_units().len(), 2);
    assert_eq!(context.yul_identifiers(), vec![&yul]);
    assert_eq!(context.yul_blocks(), vec![&block]);
    assert_eq!(context.get_source_unit_from_child_node(&block.into()), Some(&first));
    assert_eq!(context.get_source_unit_from_child_node(&yul.into()), Some(&first));
    assert_eq!(context.get_source_unit_from_child_node(&first.clone().into()), Some(&first));
}
