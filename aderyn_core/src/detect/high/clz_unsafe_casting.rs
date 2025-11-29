use std::{
    collections::{BTreeMap, HashSet},
    error::Error,
};

use crate::{
    ast::{Expression, NodeID, YulExpression},
    capture,
    context::workspace::WorkspaceContext,
    detect::detector::{IssueDetector, IssueDetectorNamePool, IssueSeverity},
};
use eyre::Result;
use solidity_ast::EvmVersion;

#[derive(Default)]
pub struct ClzUnsafeCastingDetector {
    found_instances: BTreeMap<(String, usize, String), NodeID>,
}

impl IssueDetector for ClzUnsafeCastingDetector {
    fn detect(&mut self, context: &WorkspaceContext) -> Result<bool, Box<dyn Error>> {
        // CLZ opcode only exists in Osaka hardfork and later
        if context.evm_version < EvmVersion::Osaka {
            return Ok(false);
        }

        let mut clz_variables: HashSet<(NodeID, String)> = HashSet::new();

        for assignment in context.yul_assignments() {
            if let YulExpression::YulFunctionCall(call) = &assignment.value
                && call.function_name.name == "clz"
                && let Some(node_context) = context.yul_assignments_context.get(assignment)
                && let Some(func_id) = node_context.function_definition_id
            {
                for var in &assignment.variable_names {
                    clz_variables.insert((func_id, var.name.clone()));
                }
            }
        }

        // Check for unsafe casts to uint8/int8
        for function_call in context.function_calls() {
            if function_call.kind == crate::ast::FunctionCallKind::TypeConversion {
                let is_unsafe_target =
                    if let Some(type_name) = &function_call.type_descriptions.type_string {
                        type_name == "uint8" || type_name == "int8"
                    } else {
                        // Fallback to expression checking if type_string is missing (rare)
                        if let Expression::ElementaryTypeNameExpression(etne) =
                            &*function_call.expression
                        {
                            match &etne.type_name {
                                crate::ast::TypeName::ElementaryTypeName(t) => {
                                    t.name == "uint8" || t.name == "int8"
                                }
                                crate::ast::TypeName::Raw(s) => s == "uint8" || s == "int8",
                                _ => false,
                            }
                        } else {
                            false
                        }
                    };

                if is_unsafe_target
                    && let Some(arg) = function_call.arguments.first()
                    && let Expression::Identifier(ident) = arg
                    && let Some(node_context) = context.function_calls_context.get(function_call)
                    && let Some(func_id) = node_context.function_definition_id
                    && clz_variables.contains(&(func_id, ident.name.clone()))
                {
                    capture!(self, context, function_call);
                }
            }
        }

        Ok(!self.found_instances.is_empty())
    }

    fn severity(&self) -> IssueSeverity {
        IssueSeverity::High
    }

    fn title(&self) -> String {
        String::from("Unsafe Casting of CLZ Result")
    }

    fn description(&self) -> String {
        String::from(
            "Casting the result of `clz` to `uint8` or `int8` is unsafe because `clz(0)` returns 256, which overflows these types (wrapping to 0). Ensure `x != 0` or check bounds before casting.",
        )
    }

    fn instances(&self) -> BTreeMap<(String, usize, String), NodeID> {
        self.found_instances.clone()
    }

    fn name(&self) -> String {
        format!("{}", IssueDetectorNamePool::ClzUnsafeCasting)
    }
}

#[cfg(test)]
mod clz_unsafe_casting_tests {
    use crate::{ast::ASTNode, detect::detector::IssueDetector};

    #[test]
    fn test_clz_unsafe_casting_detector() {
        let context = crate::detect::test_utils::load_solidity_source_unit(
            "../tests/contract-playground/src/CLZTypeCastingTest.sol",
        );

        let mut detector = super::ClzUnsafeCastingDetector::default();
        let found = detector.detect(&context).unwrap();
        assert!(found);

        let instances = detector.instances();

        let mut found_functions = Vec::new();
        for (_, node_id) in instances {
            if let Some(ASTNode::FunctionCall(call)) = context.nodes.get(&node_id) {
                if let Some(node_context) = context.function_calls_context.get(call) {
                    if let Some(func_id) = node_context.function_definition_id {
                        if let Some(ASTNode::FunctionDefinition(func)) = context.nodes.get(&func_id)
                        {
                            found_functions.push(func.name.clone());
                        }
                    }
                }
            }
        }

        found_functions.sort();

        // NOTE: This detector flags ALL casts of clz results to uint8/int8,
        // including safe patterns with guards (e.g., require(x != 0)).
        // This is intentional - static analyzers should be conservative and
        // flag all potentially dangerous patterns. Auditors manually verify
        // if proper guards are in place.
        let expected_functions = vec![
            "safeCastToInt8",
            "safeCastToUint8",
            "safeCastToUint8WithAssert",
            "testCasting",
            "testCasting",
            "testCastingMax",
            "testCastingOne",
            "testCastingTwo",
            "testCastingZero",
            "unsafeCastToInt8",  // Actually unsafe
            "unsafeCastToUint8", // Actually unsafe
        ];

        assert_eq!(found_functions, expected_functions);
    }
}
