use std::{
    collections::{BTreeMap, HashSet},
    error::Error,
};

use crate::{
    ast::{ASTNode, Expression, NodeID, YulBlock, YulExpression, YulStatement},
    capture,
    context::workspace::WorkspaceContext,
    detect::detector::{IssueDetector, IssueDetectorNamePool, IssueSeverity},
};
use eyre::Result;
use solidity_ast::EvmVersion;

/// Detects msb/log2/subtraction patterns based on clz(x)
/// Matches:
///   sub(K, clz(x))
///   sub(clz(x), K)
///   K - clzVar
///   clzVar - K
///   K - (expr containing clzVar)
///   (expr containing clzVar) - K
#[derive(Default)]
pub struct ClzSubtractionDetector {
    found_instances: BTreeMap<(String, usize, String), NodeID>,

    #[cfg(test)]
    found_function_names: Vec<String>,
}

impl IssueDetector for ClzSubtractionDetector {
    fn detect(&mut self, context: &WorkspaceContext) -> Result<bool, Box<dyn Error>> {
        // CLZ opcode only exists in Osaka hardfork and later
        if context.evm_version < EvmVersion::Osaka {
            return Ok(false);
        }

        let mut clz_vars: HashSet<(NodeID, String)> = HashSet::new();
        collect_yul_clz_vars(context, &mut clz_vars);

        // Check Yul sub() calls
        for yul_call in context.yul_function_calls() {
            if yul_call.function_name.name != "sub" || yul_call.arguments.len() != 2 {
                continue;
            }

            if let Some(ctx) = context.yul_function_calls_context.get(yul_call)
                && let Some(func_id) = ctx.function_definition_id
            {
                let left = &yul_call.arguments[0];
                let right = &yul_call.arguments[1];

                let is_match = yul_expr_contains_clz(right, func_id, &clz_vars, context)
                    || yul_expr_contains_clz(left, func_id, &clz_vars, context);

                if is_match {
                    capture!(self, context, yul_call);

                    #[cfg(test)]
                    record_function_name(self, context, yul_call);
                }
            }
        }

        // Check Solidity binary "-" operations
        for bin_op in context.binary_operations() {
            if bin_op.operator != "-" {
                continue;
            }

            if let Some(ctx) = context.binary_operations_context.get(bin_op)
                && let Some(func_id) = ctx.function_definition_id
            {
                let left = &bin_op.left_expression;
                let right = &bin_op.right_expression;

                let left_has_clz = solidity_expr_contains_clz(left, func_id, &clz_vars, context);
                let right_has_clz = solidity_expr_contains_clz(right, func_id, &clz_vars, context);

                if left_has_clz || right_has_clz {
                    capture!(self, context, bin_op);

                    #[cfg(test)]
                    record_function_name_op(self, context, bin_op);
                }
            }
        }

        Ok(!self.found_instances.is_empty())
    }

    fn severity(&self) -> IssueSeverity {
        IssueSeverity::High
    }

    fn title(&self) -> String {
        "CLZ Subtraction Pattern".into()
    }

    fn description(&self) -> String {
        "Patterns of the form K - clz(x), clz(x) - K, sub(K, clz(x)), \
         or subtraction involving expressions derived from clz(x). \
         clz(0) = 256 leads to wraparound in msb/log2/bitLength calculations."
            .into()
    }

    fn instances(&self) -> BTreeMap<(String, usize, String), NodeID> {
        self.found_instances.clone()
    }

    fn name(&self) -> String {
        format!("{}", IssueDetectorNamePool::ClzSubtraction)
    }
}

//
// ========================
// SUPPORT FUNCTIONS
// ========================
//

fn is_yul_clz_call(expr: &YulExpression) -> bool {
    if let YulExpression::YulFunctionCall(call) = expr {
        call.function_name.name == "clz" && call.arguments.len() == 1
    } else {
        false
    }
}

fn yul_expr_contains_clz(
    expr: &YulExpression,
    func_id: NodeID,
    clz_vars: &HashSet<(NodeID, String)>,
    context: &WorkspaceContext,
) -> bool {
    match expr {
        YulExpression::YulIdentifier(ident) => {
            // FIX: must verify variable belongs to SAME function using actual identifier context
            if let Some(ctx) = context.yul_identifiers_context.get(ident)
                && let Some(actual_func_id) = ctx.function_definition_id
            {
                return clz_vars.contains(&(actual_func_id, ident.name.clone()));
            }
            // Fallback: if identifier context missing, use func_id from parent call
            clz_vars.contains(&(func_id, ident.name.clone()))
        }

        YulExpression::YulFunctionCall(call) => {
            // direct clz(x)
            if is_yul_clz_call(expr) {
                return true;
            }

            // recursively inspect arguments
            call.arguments.iter().any(|arg| yul_expr_contains_clz(arg, func_id, clz_vars, context))
        }

        _ => false,
    }
}

// Helper trait for Expression utilities
trait ExprUtils {
    fn as_identifier_name(&self) -> Option<String>;
}

impl ExprUtils for Expression {
    fn as_identifier_name(&self) -> Option<String> {
        if let Expression::Identifier(id) = self {
            return Some(id.name.clone());
        }
        None
    }
}

fn solidity_expr_contains_clz(
    expr: &Expression,
    func_id: NodeID,
    clz_vars: &HashSet<(NodeID, String)>,
    context: &WorkspaceContext,
) -> bool {
    match expr {
        Expression::Identifier(ident) => {
            // FIX: check variable ONLY inside same function scope
            if let Some(ctx) = context.identifiers_context.get(ident)
                && let Some(actual_func_id) = ctx.function_definition_id
            {
                return clz_vars.contains(&(actual_func_id, ident.name.clone()));
            }
            // Fallback: if identifier context missing, use func_id from parent binary operation
            clz_vars.contains(&(func_id, ident.name.clone()))
        }

        Expression::FunctionCall(call) => {
            // FIX: detect Solidity's clz(x) calls (used via inline assembly)
            if let Some(name) = call.expression.as_identifier_name()
                && name == "clz"
                && call.arguments.len() == 1
            {
                return true;
            }
            // Recurse into arguments
            call.arguments.iter().any(|a| solidity_expr_contains_clz(a, func_id, clz_vars, context))
        }

        Expression::BinaryOperation(bin) => {
            solidity_expr_contains_clz(&bin.left_expression, func_id, clz_vars, context)
                || solidity_expr_contains_clz(&bin.right_expression, func_id, clz_vars, context)
        }

        Expression::UnaryOperation(un) => {
            solidity_expr_contains_clz(&un.sub_expression, func_id, clz_vars, context)
        }

        Expression::TupleExpression(tuple) => tuple
            .components
            .iter()
            .filter_map(|e| e.as_ref())
            .any(|c| solidity_expr_contains_clz(c, func_id, clz_vars, context)),

        _ => false,
    }
}

//
// Collect Yul clz variables into clz_vars
//
fn collect_yul_clz_vars(context: &WorkspaceContext, out: &mut HashSet<(NodeID, String)>) {
    // Simple Yul assignments: r := clz(x)
    for assignment in context.yul_assignments() {
        if is_yul_clz_call(&assignment.value)
            && let Some(ctx) = context.yul_assignments_context.get(assignment)
            && let Some(fid) = ctx.function_definition_id
        {
            for v in &assignment.variable_names {
                out.insert((fid, v.name.clone()));
            }
        }
    }

    // Full InlineAssembly scan (YulVariableDeclaration)
    for node in context.nodes.values() {
        if let ASTNode::InlineAssembly(assembly) = node
            && let Some(block) = &assembly.ast
        {
            let func_id = get_enclosing_function(context, assembly.id);
            if let Some(fid) = func_id {
                let mut vars = HashSet::new();
                collect_clz_vars_yul_block(block, &mut vars);
                for v in vars {
                    out.insert((fid, v));
                }
            }
        }
    }
}

fn collect_clz_vars_yul_block(block: &YulBlock, out: &mut HashSet<String>) {
    for stmt in &block.statements {
        match stmt {
            YulStatement::YulVariableDeclaration(decl) => {
                if let Some(v) = &decl.value
                    && is_yul_clz_call(v)
                {
                    for name in &decl.variables {
                        out.insert(name.name.clone());
                    }
                }
            }
            YulStatement::YulAssignment(assign) => {
                if is_yul_clz_call(&assign.value) {
                    for v in &assign.variable_names {
                        out.insert(v.name.clone());
                    }
                }
            }
            YulStatement::YulIf(y) => collect_clz_vars_yul_block(&y.body, out),
            YulStatement::YulSwitch(s) => {
                for c in &s.cases {
                    collect_clz_vars_yul_block(&c.body, out);
                }
            }
            YulStatement::YulForLoop(f) => {
                collect_clz_vars_yul_block(&f.pre, out);
                collect_clz_vars_yul_block(&f.post, out);
                collect_clz_vars_yul_block(&f.body, out);
            }
            YulStatement::YulBlock(b) => collect_clz_vars_yul_block(b, out),
            YulStatement::YulFunctionDefinition(f) => collect_clz_vars_yul_block(&f.body, out),
            _ => {}
        }
    }
}

/// Find the enclosing function for an assembly block
fn get_enclosing_function(context: &WorkspaceContext, mut node_id: NodeID) -> Option<NodeID> {
    for _ in 0..20 {
        if let Some(pid) = context.parent_link.get(&node_id) {
            if let Some(ASTNode::FunctionDefinition(func)) = context.nodes.get(pid) {
                return Some(func.id);
            }
            node_id = *pid;
        } else {
            break;
        }
    }
    None
}

//
// Helpers for tests
//
#[cfg(test)]
fn record_function_name(
    det: &mut ClzSubtractionDetector,
    context: &WorkspaceContext,
    yul_call: &crate::ast::YulFunctionCall,
) {
    if let Some(ctx) = context.yul_function_calls_context.get(yul_call) {
        if let Some(fid) = ctx.function_definition_id {
            if let Some(ASTNode::FunctionDefinition(func)) = context.nodes.get(&fid) {
                det.found_function_names.push(func.name.clone());
            }
        }
    }
}

#[cfg(test)]
fn record_function_name_op(
    det: &mut ClzSubtractionDetector,
    context: &WorkspaceContext,
    node: &crate::ast::BinaryOperation,
) {
    if let Some(ctx) = context.binary_operations_context.get(node) {
        if let Some(fid) = ctx.function_definition_id {
            if let Some(ASTNode::FunctionDefinition(func)) = context.nodes.get(&fid) {
                det.found_function_names.push(func.name.clone());
            }
        }
    }
}

#[cfg(test)]
mod clz_subtraction_tests {
    use crate::detect::detector::IssueDetector;

    #[test]
    fn test_clz_subtraction_detector() {
        let context = crate::detect::test_utils::load_solidity_source_unit(
            "../tests/contract-playground/src/CLZSubtractionPatternTest.sol",
        );

        let mut detector = super::ClzSubtractionDetector::default();
        let found = detector.detect(&context).unwrap();
        assert!(found, "Should detect subtraction patterns");

        let instances = detector.instances();

        let mut found_functions = detector.found_function_names.clone();
        found_functions.sort();

        // Expected functions with sub(K, clz(x)), sub(clz(x), K), K - clz, or clz - K patterns.
        // NOTE: This detector conservatively flags ALL subtraction patterns involving CLZ,
        // including safe implementations with guards (e.g., safeMsb, safeBitLength).
        // Auditors must manually verify if proper zero-checks are in place.
        let expected_functions = vec![
            "brokenArrayAccess",
            "brokenBitLength",
            "brokenBitLengthSolidity",
            "brokenCalculationSolidity",
            "brokenComplexExpression1",
            "brokenComplexExpression2",
            "brokenComplexExpression3",
            "brokenInitialGuess",
            "brokenLog2",
            "brokenLog2Solidity",
            "brokenMsb",
            "brokenMsbSolidity",
            "brokenNormalization",
            "brokenPriceNormalization",
            "brokenReverseExpression",
            "brokenReverseSolidity",
            "brokenReverseYul",
            "demonstrateSqrtFailure",
            "safeBitLength",
            "safeBitLengthSolidity",
            "safeInitialGuess",
            "safeLog2",
            "safeMsb",
            "safeMsbSolidityIf",
            "safeMsbSolidityRequire",
            "safeMsbSolidityUnchecked",
            "safePriceNormalization",
            "testSub255",
            "testSub256",
            "testSubWithConstant",
        ];

        println!("Found {} instances", instances.len());
        println!("Found functions: {:?}", found_functions);

        assert_eq!(found_functions.len(), expected_functions.len());
        assert_eq!(found_functions, expected_functions);
    }
}
