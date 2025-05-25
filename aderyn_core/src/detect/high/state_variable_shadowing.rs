use std::{
    collections::{BTreeMap, HashMap},
    error::Error,
};

use crate::ast::{
    ContractDefinition, Mutability, NodeID, NodeType, UserDefinedTypeNameOrIdentifierPath,
    VariableDeclaration,
};

use crate::{
    capture,
    context::{
        browser::{
            ExtractPragmaDirectives, ExtractVariableDeclarations, GetClosestAncestorOfTypeX,
        },
        workspace::{ASTNode, WorkspaceContext},
    },
    detect::{
        detector::{IssueDetector, IssueDetectorNamePool, IssueSeverity},
        helpers::pragma_directive_to_semver,
    },
};
use eyre::Result;
use semver::VersionReq;

// This detector catches an issue that is only present on contracts that can be compiled
// with Solidity version < 0.6.0. In Solidity 0.6.0, the `override` keyword was introduced
// to override state variables from contracts that are inherited and extended.

// Preprocessing that would make this detector more efficient:
// 1. Inheritance/Extension Tree
// 2. Solc version based detector assignment (if solc version < 0.6.0, run this detector on the
//    workspace context)

#[derive(Default)]
pub struct StateVariableShadowingDetector {
    // Keys are: [0] source file name, [1] line number, [2] character location of node.
    // Do not add items manually, use `capture!` to add nodes to this BTreeMap.
    found_instances: BTreeMap<(String, usize, String), NodeID>,
}

fn are_duplicate_names_in_inherited_contracts(
    context: &WorkspaceContext,
    variable_name: &str, // Use &str directly for comparison efficiency
    contract_definition: &ContractDefinition, // Use reference to avoid cloning
) -> bool {
    // Check for duplicate variable names in the current contract
    if ExtractVariableDeclarations::from(contract_definition).extracted.iter().any(|vd| {
        vd.state_variable
            && vd.mutability() != Some(&Mutability::Constant)
            && vd.name == variable_name
    }) {
        return true; // Return immediately if a duplicate is found
    }

    // Recursively check base contracts
    for base_contract in &contract_definition.base_contracts {
        match &base_contract.base_name {
            UserDefinedTypeNameOrIdentifierPath::UserDefinedTypeName(base_name) => {
                if let Some(ASTNode::ContractDefinition(contract)) =
                    context.nodes.get(&base_name.referenced_declaration)
                {
                    if are_duplicate_names_in_inherited_contracts(context, variable_name, contract)
                    {
                        return true; // Return immediately if a duplicate is found
                    }
                }
            }
            UserDefinedTypeNameOrIdentifierPath::IdentifierPath(identifier_path) => {
                if let Some(ASTNode::ContractDefinition(contract)) =
                    context.nodes.get(&(identifier_path.referenced_declaration))
                {
                    if are_duplicate_names_in_inherited_contracts(context, variable_name, contract)
                    {
                        return true; // Return immediately if a duplicate is found
                    }
                }
            }
        }
    }

    false // Return false if no duplicates found
}

fn source_unit_can_compile_below_0_6_0(context: &WorkspaceContext, child_node_id: NodeID) -> bool {
    let source_unit_ast = context.get_closest_ancestor(child_node_id, NodeType::SourceUnit);

    if let Some(ASTNode::SourceUnit(source_unit)) = source_unit_ast {
        // Store the extracted directives in a variable to extend its lifetime
        let extracted_directives = ExtractPragmaDirectives::from(source_unit).extracted;
        let pragma_directive = extracted_directives.first();

        if let Some(pragma_directive) = pragma_directive {
            let version_req = pragma_directive_to_semver(pragma_directive);
            if let Ok(version_req) = version_req {
                return allows_below_0_6_0(&version_req);
            }
        }
    }
    false
}

fn allows_below_0_6_0(version_req: &VersionReq) -> bool {
    if version_req.comparators.is_empty() {
        return false; // Return false or handle as needed if there are no comparators
    }

    let comparator = &version_req.comparators[0];
    comparator.major == 0 && comparator.minor.is_some_and(|m| m < 6)
}

impl IssueDetector for StateVariableShadowingDetector {
    fn detect(&mut self, context: &WorkspaceContext) -> Result<bool, Box<dyn Error>> {
        let mut temp_map: HashMap<String, Vec<&VariableDeclaration>> = context
            .variable_declarations()
            .into_iter()
            .filter(|vd| {
                vd.state_variable
                    && !vd.constant
                    && source_unit_can_compile_below_0_6_0(context, vd.id)
            })
            .fold(HashMap::new(), |mut acc, var| {
                acc.entry(var.name.clone()).or_default().push(var);
                acc
            });

        // Filter the map to only include entries with more than one variable
        temp_map.retain(|_, v| v.len() > 1);

        for (_, variables) in temp_map {
            for variable in variables {
                // Recurse up the inheritance tree
                let contract_ast =
                    variable.closest_ancestor_of_type(context, NodeType::ContractDefinition);
                if let Some(ASTNode::ContractDefinition(contract)) = contract_ast {
                    for base_contract in &contract.base_contracts {
                        match &base_contract.base_name {
                            UserDefinedTypeNameOrIdentifierPath::UserDefinedTypeName(base_name) => {
                                if let Some(ASTNode::ContractDefinition(contract)) =
                                    context.nodes.get(&base_name.referenced_declaration)
                                {
                                    if are_duplicate_names_in_inherited_contracts(
                                        context,
                                        &variable.name,
                                        contract,
                                    ) {
                                        capture!(self, context, variable);
                                    }
                                }
                            }
                            UserDefinedTypeNameOrIdentifierPath::IdentifierPath(
                                identifier_path,
                            ) => {
                                if let Some(ASTNode::ContractDefinition(contract)) =
                                    context.nodes.get(&(identifier_path.referenced_declaration))
                                {
                                    if are_duplicate_names_in_inherited_contracts(
                                        context,
                                        &variable.name,
                                        contract,
                                    ) {
                                        capture!(self, context, variable);
                                    }
                                }
                            }
                        };
                    }
                }
            }
        }

        Ok(!self.found_instances.is_empty())
    }

    fn severity(&self) -> IssueSeverity {
        IssueSeverity::High
    }

    fn title(&self) -> String {
        String::from("Shadowed State Variable")
    }

    fn description(&self) -> String {
        String::from(
            "This vulnerability arises when a derived contract unintentionally shadows a state variable from \
            a parent contract by declaring a variable with the same name. This can be misleading. \
            To prevent this, ensure variable names \
            are unique across the inheritance hierarchy or use proper visibility and scope controls."
        )
    }

    fn instances(&self) -> BTreeMap<(String, usize, String), NodeID> {
        self.found_instances.clone()
    }

    fn name(&self) -> String {
        IssueDetectorNamePool::StateVariableShadowing.to_string()
    }
}

#[cfg(test)]
mod state_variable_shadowing_detector_tests {

    use crate::detect::{detector::IssueDetector, high::StateVariableShadowingDetector};

    #[test]

    fn test_state_variable_shadowing_detector() {
        let context = crate::detect::test_utils::load_solidity_source_unit(
            "../tests/contract-playground/src/StateShadowing.sol",
        );

        let mut detector = StateVariableShadowingDetector::default();
        let found = detector.detect(&context).unwrap();
        assert!(found);
        assert_eq!(detector.instances().len(), 1);
    }
}
