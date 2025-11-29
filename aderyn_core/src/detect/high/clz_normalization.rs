use std::{
    collections::{BTreeMap, HashSet},
    error::Error,
};

use crate::{
    ast::{ASTNode, NodeID, YulBlock, YulExpression, YulStatement},
    capture,
    context::workspace::WorkspaceContext,
    detect::detector::{IssueDetector, IssueDetectorNamePool, IssueSeverity},
};
use eyre::Result;
use solidity_ast::EvmVersion;

/// Detects the normalization pattern:
///     let r := clz(x)
///     shr(C, shl(r, x))
/// which is unsafe when x == 0
#[derive(Default)]
pub struct ClzNormalizationDetector {
    found_instances: BTreeMap<(String, usize, String), NodeID>,
    #[cfg(test)]
    found_function_names: Vec<String>,
}

impl IssueDetector for ClzNormalizationDetector {
    fn detect(&mut self, context: &WorkspaceContext) -> Result<bool, Box<dyn Error>> {
        // CLZ opcode only exists in Osaka hardfork and later
        if context.evm_version < EvmVersion::Osaka {
            return Ok(false);
        }

        // Track variables assigned from clz(x)
        let mut clz_vars: HashSet<(NodeID, String)> = HashSet::new();

        // Collect clz-assigned variables from inline assembly blocks
        for node in context.nodes.values() {
            if let ASTNode::InlineAssembly(assembly) = node
                && let Some(block) = &assembly.ast
            {
                // Find parent function definition
                let mut parent_id = assembly.id;
                let mut func_id_opt = None;

                for _ in 0..20 {
                    if let Some(parent) = context.get_parent(parent_id) {
                        if let ASTNode::FunctionDefinition(func) = parent {
                            func_id_opt = Some(func.id);
                            break;
                        }
                        if let Some(pid) = context.parent_link.get(&parent_id) {
                            parent_id = *pid;
                        } else {
                            break;
                        }
                    } else {
                        break;
                    }
                }

                if let Some(func_id) = func_id_opt {
                    let mut vars = HashSet::new();
                    collect_clz_vars(block, &mut vars);
                    for var in vars {
                        clz_vars.insert((func_id, var));
                    }
                }
            }
        }

        // 2. Scan for pattern:  shr(C, shl(r, x))
        for yul_call in context.yul_function_calls() {
            if yul_call.function_name.name != "shr" || yul_call.arguments.len() != 2 {
                continue;
            }

            // second argument must be shl(...)
            let shl_expr = match &yul_call.arguments[1] {
                YulExpression::YulFunctionCall(inner)
                    if inner.function_name.name == "shl" && inner.arguments.len() == 2 =>
                {
                    inner
                }
                _ => continue,
            };

            let shl_first = &shl_expr.arguments[0];
            let shl_second = &shl_expr.arguments[1];

            let mut is_clz_shift = false;

            if let YulExpression::YulFunctionCall(c) = shl_first
                && c.function_name.name == "clz"
                && c.arguments.len() == 1
                && are_yul_expressions_equal(&c.arguments[0], shl_second)
            {
                is_clz_shift = true;
            }

            // Check indirect case: shl(r, x) where r := clz(x)
            if let YulExpression::YulIdentifier(ident_r) = shl_first
                && let Some(ctx) = context.yul_function_calls_context.get(yul_call)
                && let Some(func_id) = ctx.function_definition_id
                && clz_vars.contains(&(func_id, ident_r.name.clone()))
            {
                is_clz_shift = true;
            }

            if is_clz_shift {
                capture!(self, context, yul_call);

                #[cfg(test)]
                {
                    if let Some(ctx) = context.yul_function_calls_context.get(yul_call) {
                        if let Some(func_id) = ctx.function_definition_id {
                            if let Some(ASTNode::FunctionDefinition(func)) =
                                context.nodes.get(&func_id)
                            {
                                self.found_function_names.push(func.name.clone());
                            }
                        }
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
        String::from("CLZ Normalization Pattern")
    }

    fn description(&self) -> String {
        String::from(
            "Detected lnWad-style normalization pattern `shr(C, shl(r, x))` using `clz(x)`. \
        When x == 0, clz(x) = 256 which produces garbage values and breaks ln/exp normalization logic.",
        )
    }

    fn instances(&self) -> BTreeMap<(String, usize, String), NodeID> {
        self.found_instances.clone()
    }

    fn name(&self) -> String {
        format!("{}", IssueDetectorNamePool::ClzNormalization)
    }
}

fn collect_clz_vars(block: &YulBlock, vars: &mut HashSet<String>) {
    for stmt in &block.statements {
        match stmt {
            YulStatement::YulVariableDeclaration(decl) => {
                if let Some(value) = &decl.value
                    && is_clz_call(value)
                {
                    for var in &decl.variables {
                        vars.insert(var.name.clone());
                    }
                }
            }
            YulStatement::YulAssignment(assign) => {
                if is_clz_call(&assign.value) {
                    for var in &assign.variable_names {
                        vars.insert(var.name.clone());
                    }
                }
            }
            YulStatement::YulIf(yul_if) => collect_clz_vars(&yul_if.body, vars),
            YulStatement::YulSwitch(yul_switch) => {
                for case in &yul_switch.cases {
                    collect_clz_vars(&case.body, vars);
                }
            }
            YulStatement::YulForLoop(yul_for) => {
                collect_clz_vars(&yul_for.pre, vars);
                collect_clz_vars(&yul_for.post, vars);
                collect_clz_vars(&yul_for.body, vars);
            }
            YulStatement::YulBlock(yul_block) => collect_clz_vars(yul_block, vars),
            YulStatement::YulFunctionDefinition(func) => collect_clz_vars(&func.body, vars),
            _ => {}
        }
    }
}

fn is_clz_call(expr: &YulExpression) -> bool {
    if let YulExpression::YulFunctionCall(call) = expr {
        call.function_name.name == "clz" && call.arguments.len() == 1
    } else {
        false
    }
}

fn are_yul_expressions_equal(a: &YulExpression, b: &YulExpression) -> bool {
    match (a, b) {
        (YulExpression::YulIdentifier(id1), YulExpression::YulIdentifier(id2)) => {
            id1.name == id2.name
        }
        _ => false,
    }
}

#[cfg(test)]
mod clz_normalization_tests {
    use crate::detect::detector::IssueDetector;

    #[test]
    fn test_clz_normalization_detector() {
        let context = crate::detect::test_utils::load_solidity_source_unit(
            "../tests/contract-playground/src/CLZNormalizationPatternTest.sol",
        );

        let mut detector = super::ClzNormalizationDetector::default();
        let found = detector.detect(&context).unwrap();
        assert!(found, "Should detect normalization patterns");

        let instances = detector.instances();

        let mut found_functions = detector.found_function_names.clone();
        found_functions.sort();

        // Expected functions that contain the shr(C, shl(r, x)) pattern.
        // Note: safeLnWad is NOT here because it only calculates k, it doesn't shift x.
        let expected_functions = vec![
            "brokenAmmPrice",
            "brokenExpNormalization",
            "brokenInterestRate",
            "brokenLnWad",
            "brokenNormalize",
            "compareNormalization",
            "demonstrateEconomicImpact",
            "demonstrateLnWadPattern",
            "safeNormalize",
            "safeNormalizeAsm",
            "testCommonConstants",
            "testCommonConstants", // Contains two instances
            "testNormalizationWithConstant",
        ];

        assert_eq!(found_functions, expected_functions);
    }
}
