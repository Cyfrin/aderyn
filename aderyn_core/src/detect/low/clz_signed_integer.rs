use std::{
    collections::{BTreeMap, HashMap},
    error::Error,
};

use crate::{
    ast::{NodeID, YulExpression},
    capture,
    context::workspace::WorkspaceContext,
    detect::detector::{IssueDetector, IssueDetectorNamePool, IssueSeverity},
};
use eyre::Result;
use solidity_ast::EvmVersion;

#[derive(Default)]
pub struct ClzSignedIntegerDetector {
    found_instances: BTreeMap<(String, usize, String), NodeID>,
}

impl IssueDetector for ClzSignedIntegerDetector {
    fn detect(&mut self, context: &WorkspaceContext) -> Result<bool, Box<dyn Error>> {
        // CLZ opcode only exists in Osaka hardfork and later
        if context.evm_version < EvmVersion::Osaka {
            return Ok(false);
        }

        // Build a map of (FunctionID, VariableName) -> TypeString
        let mut var_types: HashMap<(NodeID, String), String> = HashMap::new();

        // Collect variable type information
        for var in context.variable_declarations() {
            if let Some(node_context) = context.variable_declarations_context.get(var)
                && let Some(func_id) = node_context.function_definition_id
                && let Some(type_string) = &var.type_descriptions.type_string
            {
                var_types.insert((func_id, var.name.clone()), type_string.clone());
            }
        }

        // Collect function parameter types
        for func in context.function_definitions() {
            for param in &func.parameters.parameters {
                if let Some(type_string) = &param.type_descriptions.type_string {
                    var_types.insert((func.id, param.name.clone()), type_string.clone());
                }
            }
        }

        // Check clz calls
        for yul_call in context.yul_function_calls() {
            if yul_call.function_name.name == "clz"
                && yul_call.arguments.len() == 1
                && let Some(node_context) = context.yul_function_calls_context.get(yul_call)
                && let Some(func_id) = node_context.function_definition_id
                && check_yul_expr_for_signed(&yul_call.arguments[0], &var_types, func_id)
            {
                capture!(self, context, yul_call);
            }
        }
        Ok(!self.found_instances.is_empty())
    }

    fn severity(&self) -> IssueSeverity {
        IssueSeverity::Low
    }

    fn title(&self) -> String {
        String::from("CLZ used with Signed Integer")
    }

    fn description(&self) -> String {
        String::from(
            "Using `clz` with a signed integer (`int256`, etc.) treats the value as unsigned. Negative numbers (in two's complement) start with `1`, so `clz` will return 0, which might be unexpected. Ensure you handle the sign or cast to unsigned explicitly if this is intended.",
        )
    }

    fn instances(&self) -> BTreeMap<(String, usize, String), NodeID> {
        self.found_instances.clone()
    }

    fn name(&self) -> String {
        format!("{}", IssueDetectorNamePool::ClzSignedInteger)
    }
}

fn check_yul_expr_for_signed(
    expr: &YulExpression,
    var_types: &HashMap<(NodeID, String), String>,
    func_id: NodeID,
) -> bool {
    match expr {
        YulExpression::YulIdentifier(ident) => {
            if let Some(type_string) = var_types.get(&(func_id, ident.name.clone())) {
                // Check if type is signed integer (int...)
                if type_string.starts_with("int") && !type_string.starts_with("uint") {
                    return true;
                }
            }
        }
        YulExpression::YulFunctionCall(call) => {
            // Check for explicit casts to unsigned (e.g. uint256(x))
            if call.function_name.name.starts_with("uint") {
                return false; // Explicit cast to unsigned -> Safe
            }

            // Check for explicit casts to signed (e.g. int256(x))
            if call.function_name.name.starts_with("int") {
                return true; // Explicit cast to signed -> Unsafe
            }

            // Check for operations that preserve or introduce signedness
            // Arithmetic: add, sub, mul - propagate signedness of operands
            if matches!(call.function_name.name.as_str(), "add" | "sub" | "mul") {
                for arg in &call.arguments {
                    if check_yul_expr_for_signed(arg, var_types, func_id) {
                        return true;
                    }
                }
            }

            // Signed division/modulo - result is signed
            if matches!(call.function_name.name.as_str(), "sdiv" | "smod") {
                return true;
            }

            // Sign extend - result is signed
            if call.function_name.name == "signextend" {
                return true;
            }

            // For other calls, we assume they might return unsigned unless proven otherwise
            // (Standard Yul functions like and, or, xor, not, byte, shl, shr, sar usually treat
            // bits as bits) Note: `sar` (arithmetic shift right) preserves sign bit,
            // but result is still just bits. If we are strict, `sar` on a signed value
            // keeps it signed.
            if call.function_name.name == "sar" {
                // If the value being shifted is signed, the result is signed
                if call.arguments.len() == 2
                    && check_yul_expr_for_signed(&call.arguments[1], var_types, func_id)
                {
                    return true;
                }
            }
        }
        _ => {}
    }
    false
}

#[cfg(test)]
mod clz_signed_integer_tests {
    use crate::{ast::ASTNode, detect::detector::IssueDetector};

    #[test]
    fn test_clz_signed_integer_detector() {
        let context = crate::detect::test_utils::load_solidity_source_unit(
            "../tests/contract-playground/src/CLZSignedIntegerTest.sol",
        );

        let mut detector = super::ClzSignedIntegerDetector::default();
        let found = detector.detect(&context).unwrap();
        assert!(found);

        let instances = detector.instances();

        // Safe functions that should NOT be detected
        let safe_functions = vec![
            "safeBitLengthInt256",
            "safeBitLengthAbsInt256",
            "safeBitLengthExplicitInt256",
            "safeConvertAndCheck",
            "safeCastExpression",
        ];

        for (_, node_id) in instances {
            // Retrieve the ASTNode from the context using the NodeID
            if let Some(ASTNode::YulFunctionCall(yul_call)) = context.nodes.get(&node_id) {
                // Get the context for this call to find the enclosing function
                if let Some(node_context) = context.yul_function_calls_context.get(yul_call) {
                    if let Some(func_id) = node_context.function_definition_id {
                        // Get the function definition to check its name
                        if let Some(ASTNode::FunctionDefinition(func)) = context.nodes.get(&func_id)
                        {
                            let func_name = &func.name;
                            assert!(
                                !safe_functions.contains(&func_name.as_str()),
                                "Found instance in safe function: {}",
                                func_name
                            );
                        }
                    }
                }
            }
        }
    }
}
