use std::{collections::BTreeMap, error::Error};

use crate::{
    ast::{BlockOrStatement, Expression, NodeID, YulExpression, YulStatement},
    capture,
    context::workspace::WorkspaceContext,
    detect::detector::{IssueDetector, IssueDetectorNamePool, IssueSeverity},
};
use eyre::Result;
use solidity_ast::EvmVersion;

#[derive(Default)]
pub struct ClzLegacyImplementationsDetector {
    found_instances: BTreeMap<(String, usize, String), NodeID>,
}

impl IssueDetector for ClzLegacyImplementationsDetector {
    fn detect(&mut self, context: &WorkspaceContext) -> Result<bool, Box<dyn Error>> {
        // CLZ opcode only exists in Osaka hardfork and later
        if context.evm_version < EvmVersion::Osaka {
            return Ok(false);
        }

        // Pattern 1: EIP-7939 fallback by magic constants
        detect_eip7939_magic(self, context);

        // Pattern 2: Assembly binary search (Uniswap / Solidity-by-example style)
        detect_assembly_binary_msb(self, context);

        // Pattern 3 & 4: Solidity-level MSB/log2/bitLength implementations
        // Semantic pattern: (shift X) + (accumulate R) + (return R/expr(R))
        detect_solidity_legacy_msb(self, context);

        // Pattern 5: Wrappers (functions calling detected legacy implementations)
        detect_legacy_wrappers(self, context);

        Ok(!self.found_instances.is_empty())
    }

    fn severity(&self) -> IssueSeverity {
        IssueSeverity::Low
    }

    fn title(&self) -> String {
        String::from("Legacy CLZ/MSB Implementation Detected")
    }

    fn description(&self) -> String {
        String::from(
            "Detected a legacy implementation of CLZ (count leading zeros), MSB (most significant bit), or log2 calculation. \
            These patterns include: (1) Uniswap/PRBMath-style binary search with `if (x >= 2**N) { x >>= N; r += N; }`, \
            (2) Linear bit scan with `while ((x >>= 1) > 0) { counter++; }`, \
            (3) Assembly binary search with masks and `shl/gt/shr/or`, (4) EIP-7939 Solidity fallback with magic constants. \
            On EVMs that support the CLZ opcode (Osaka+), consider replacing these with a single `clz` call for better gas efficiency and simpler bytecode.",
        )
    }

    fn instances(&self) -> BTreeMap<(String, usize, String), NodeID> {
        self.found_instances.clone()
    }

    fn name(&self) -> String {
        format!("{}", IssueDetectorNamePool::ClzLegacyImplementations)
    }
}

// ==========================
// Pattern 1: EIP-7939 magic
// ==========================

fn detect_eip7939_magic(
    detector: &mut ClzLegacyImplementationsDetector,
    context: &WorkspaceContext,
) {
    use crate::context::browser::ExtractInlineAssemblies;

    for func in context.function_definitions() {
        if let Some(body) = &func.body {
            let assemblies = ExtractInlineAssemblies::from(body).extracted;
            for asm in assemblies {
                if let Some(yul_block) = &asm.ast {
                    for stmt in &yul_block.statements {
                        if contains_magic_constant(stmt) {
                            capture!(detector, context, func);
                            break;
                        }
                    }
                }
            }
        }
    }
}

fn contains_magic_constant(stmt: &YulStatement) -> bool {
    match stmt {
        YulStatement::YulVariableDeclaration(decl) => {
            if let Some(val) = &decl.value
                && check_yul_expr_for_magic(val)
            {
                return true;
            }
        }
        YulStatement::YulAssignment(assign) => {
            if check_yul_expr_for_magic(&assign.value) {
                return true;
            }
        }
        YulStatement::YulExpressionStatement(expr_stmt) => {
            if check_yul_expr_for_magic(&expr_stmt.expression) {
                return true;
            }
        }
        YulStatement::YulBlock(block) => {
            for nested in &block.statements {
                if contains_magic_constant(nested) {
                    return true;
                }
            }
        }
        YulStatement::YulIf(yul_if) => {
            if check_yul_expr_for_magic(&yul_if.condition) {
                return true;
            }
            for nested in &yul_if.body.statements {
                if contains_magic_constant(nested) {
                    return true;
                }
            }
        }
        YulStatement::YulSwitch(yul_switch) => {
            if check_yul_expr_for_magic(&yul_switch.expression) {
                return true;
            }
            for case in &yul_switch.cases {
                for nested in &case.body.statements {
                    if contains_magic_constant(nested) {
                        return true;
                    }
                }
            }
        }
        YulStatement::YulForLoop(yul_for) => {
            for nested in &yul_for.pre.statements {
                if contains_magic_constant(nested) {
                    return true;
                }
            }
            // Condition is YulExpression, not Option<YulExpression>
            if check_yul_expr_for_magic(&yul_for.condition) {
                return true;
            }

            for nested in &yul_for.post.statements {
                if contains_magic_constant(nested) {
                    return true;
                }
            }
            for nested in &yul_for.body.statements {
                if contains_magic_constant(nested) {
                    return true;
                }
            }
        }
        _ => {}
    }
    false
}

fn check_yul_expr_for_magic(expr: &YulExpression) -> bool {
    match expr {
        YulExpression::YulLiteral(lit) => {
            if let Some(value) = &lit.value {
                let val_lower = value.to_lowercase();
                // EIP-7939 magic constants
                // 0x8421084210842108cc6318c6db6d54be
                // 0xf8f9f9faf9fdfafbf9fdfcfdfafbfcfe
                if val_lower.contains("8421084210842108cc6318c6db6d54be")
                    || val_lower.contains("f8f9f9faf9fdfafbf9fdfcfdfafbfcfe")
                {
                    return true;
                }
            }
        }
        YulExpression::YulFunctionCall(call) => {
            return check_yul_call_for_magic(call);
        }
        _ => {}
    }
    false
}

fn check_yul_call_for_magic(call: &crate::ast::YulFunctionCall) -> bool {
    for arg in &call.arguments {
        if check_yul_expr_for_magic(arg) {
            return true;
        }
    }
    false
}

// ==============================================
// Pattern 2: Assembly binary search MSB (Yul)
// shl(7, gt(x, 0xFFFF...)); x := shr(f, x); msb := or(msb, f)
// ==============================================

fn detect_assembly_binary_msb(
    detector: &mut ClzLegacyImplementationsDetector,
    context: &WorkspaceContext,
) {
    use crate::context::browser::ExtractInlineAssemblies;

    for func in context.function_definitions() {
        // Skip functions already using clz
        if function_uses_clz(context, func.id) {
            continue;
        }

        let Some(body) = &func.body else { continue };

        // Extract all inline assembly blocks
        let assemblies = ExtractInlineAssemblies::from(body).extracted;

        // Count characteristic patterns across ALL assembly blocks in the function
        let mut mask_comparisons = 0;
        let mut has_shl_pattern = false;
        let mut has_or_pattern = false;
        let mut has_shr_pattern = false;
        let mut has_lt_pattern = false;

        for asm in assemblies {
            // Get YulBlock from InlineAssembly
            let Some(yul_block) = &asm.ast else { continue };

            // Scan Yul statements
            for yul_stmt in &yul_block.statements {
                // Count gt() calls with characteristic masks
                if contains_mask_comparison_in_yul_stmt(yul_stmt) {
                    mask_comparisons += 1;
                }
                // Check for shl pattern
                if contains_yul_call_in_stmt(yul_stmt, "shl") {
                    has_shl_pattern = true;
                }
                // Check for or pattern
                if contains_yul_call_in_stmt(yul_stmt, "or") {
                    has_or_pattern = true;
                }
                // Check for shr pattern
                if contains_yul_call_in_stmt(yul_stmt, "shr") {
                    has_shr_pattern = true;
                }
                // Check for lt pattern (used in EIP-7939 fallback)
                if contains_yul_call_in_stmt(yul_stmt, "lt") {
                    has_lt_pattern = true;
                }
            }
        }

        // If we have 2+ mask comparisons AND the characteristic shl/or/shr pattern
        // OR if we have the EIP-7939 fallback pattern which uses lt() and masks
        if mask_comparisons >= 2
            && has_shl_pattern
            && has_or_pattern
            && (has_shr_pattern || has_lt_pattern)
        {
            capture!(detector, context, func);
        }
    }
}

// Helper: check if Yul statement contains gt() with characteristic mask
fn contains_mask_comparison_in_yul_stmt(stmt: &YulStatement) -> bool {
    match stmt {
        YulStatement::YulVariableDeclaration(decl) => {
            if let Some(YulExpression::YulFunctionCall(call)) = &decl.value {
                return is_mask_comparison_call(call);
            }
        }
        YulStatement::YulAssignment(assign) => {
            if let YulExpression::YulFunctionCall(call) = &assign.value {
                return is_mask_comparison_call(call);
            }
        }
        YulStatement::YulBlock(block) => {
            // Recursively check nested blocks
            for nested_stmt in &block.statements {
                if contains_mask_comparison_in_yul_stmt(nested_stmt) {
                    return true;
                }
            }
        }
        _ => {}
    }
    false
}

fn is_mask_comparison_call(call: &crate::ast::YulFunctionCall) -> bool {
    // Check for gt(x, MASK)
    if call.function_name.name == "gt"
        && call.arguments.len() == 2
        && let YulExpression::YulLiteral(lit) = &call.arguments[1]
    {
        return is_characteristic_mask(lit);
    }

    // Check for lt(MASK, x) - used in EIP-7939 fallback
    if call.function_name.name == "lt"
        && call.arguments.len() == 2
        && let YulExpression::YulLiteral(lit) = &call.arguments[0]
    {
        return is_characteristic_mask(lit);
    }

    // Check nested: shl(k, gt(x, MASK)) or shl(k, lt(MASK, x))
    if call.function_name.name == "shl"
        && call.arguments.len() == 2
        && let YulExpression::YulFunctionCall(inner) = &call.arguments[1]
    {
        // Check inner gt(x, MASK)
        if inner.function_name.name == "gt"
            && inner.arguments.len() == 2
            && let YulExpression::YulLiteral(lit) = &inner.arguments[1]
        {
            return is_characteristic_mask(lit);
        }
        // Check inner lt(MASK, x)
        if inner.function_name.name == "lt"
            && inner.arguments.len() == 2
            && let YulExpression::YulLiteral(lit) = &inner.arguments[0]
        {
            return is_characteristic_mask(lit);
        }
    }

    false
}

fn is_characteristic_mask(lit: &crate::ast::YulLiteral) -> bool {
    if let Some(value) = &lit.value {
        let val_lower = value.to_lowercase();
        matches!(
            val_lower.as_str(),
            "0xffffffffffffffffffffffffffffffff"
                | "0xffffffffffffffff"
                | "0xffffffff"
                | "0xffff"
                | "0xff"
                | "0xf"
                | "0x3"
                | "0x1"
        )
    } else {
        false
    }
}

fn contains_yul_call_in_stmt(stmt: &YulStatement, name: &str) -> bool {
    match stmt {
        YulStatement::YulVariableDeclaration(decl) => {
            if let Some(YulExpression::YulFunctionCall(call)) = &decl.value {
                return call.function_name.name == name || contains_nested_yul_call(call, name);
            }
        }
        YulStatement::YulAssignment(assign) => {
            if let YulExpression::YulFunctionCall(call) = &assign.value {
                return call.function_name.name == name || contains_nested_yul_call(call, name);
            }
        }
        YulStatement::YulBlock(block) => {
            for nested_stmt in &block.statements {
                if contains_yul_call_in_stmt(nested_stmt, name) {
                    return true;
                }
            }
        }
        _ => {}
    }
    false
}

fn contains_nested_yul_call(call: &crate::ast::YulFunctionCall, name: &str) -> bool {
    for arg in &call.arguments {
        if let YulExpression::YulFunctionCall(nested) = arg {
            if nested.function_name.name == name {
                return true;
            }
            if contains_nested_yul_call(nested, name) {
                return true;
            }
        }
    }
    false
}

// Helper: check if expression contains power-of-two comparison
fn contains_power_of_two_comparison(expr: &Expression) -> bool {
    use crate::context::browser::ExtractBinaryOperations;

    let bin_ops = ExtractBinaryOperations::from(expr).extracted;

    for op in bin_ops {
        if matches!(op.operator.as_str(), ">=" | ">" | "<" | "<=") {
            // Check right side for 2**N pattern
            if is_power_of_two_expr(op.right_expression.as_ref()) {
                return true;
            }
            // Check left side for 2**N pattern
            if is_power_of_two_expr(op.left_expression.as_ref()) {
                return true;
            }
        }
    }
    false
}

// Helper: check if expression is a power of two (2**N or hex literal)
fn is_power_of_two_expr(expr: &Expression) -> bool {
    match expr {
        Expression::BinaryOperation(bin_op) => {
            // Check for 2**N pattern
            if bin_op.operator == "**"
                && let Expression::Literal(left_lit) = bin_op.left_expression.as_ref()
                && let Some(val) = &left_lit.value
                && val == "2"
            {
                // Any power of 2 is suspicious in this context
                return true;
            }
        }
        Expression::Literal(lit) => {
            // Check for hex literals that are powers of two
            if let Some(value) = &lit.value
                && (value.starts_with("0x") || value.starts_with("0X"))
            {
                let hex_part = &value[2..];
                if let Ok(num) = u128::from_str_radix(hex_part, 16) {
                    return num > 0 && (num & (num - 1)) == 0;
                }
                // Fallback for very large numbers (larger than u128)
                // Just check if it has exactly one non-zero bit
                // Hex: 1, 2, 4, 8 followed by 0s
                let first_char = hex_part.chars().next().unwrap_or('0');
                if ['1', '2', '4', '8'].contains(&first_char) {
                    let rest = &hex_part[1..];
                    if rest.chars().all(|c| c == '0') {
                        return true;
                    }
                }
            }
        }
        _ => {}
    }
    false
}

// ====================================================
// Pattern 5: Wrappers
// Functions that call already detected legacy implementations
// ====================================================

fn detect_legacy_wrappers(
    detector: &mut ClzLegacyImplementationsDetector,
    context: &WorkspaceContext,
) {
    use crate::context::browser::ExtractFunctionCalls;

    let mut changed = true;
    while changed {
        changed = false;
        // Snapshot current detected IDs to avoid borrow issues
        let current_detected_ids: Vec<NodeID> =
            detector.found_instances.values().cloned().collect();

        for func in context.function_definitions() {
            // If already detected, skip
            if current_detected_ids.contains(&func.id) {
                continue;
            }

            // Check if it calls any detected function
            if let Some(body) = &func.body {
                let calls = ExtractFunctionCalls::from(body).extracted;
                for call in calls {
                    // Check direct calls: legacyFunc(x)
                    if let Expression::Identifier(id) = call.expression.as_ref()
                        && let Some(ref_id) = id.referenced_declaration
                        && current_detected_ids.contains(&ref_id)
                    {
                        capture!(detector, context, func);
                        changed = true;
                        break;
                    }
                }
            }
        }
    }
}

// ====================================================
// Pattern 3 & 4: Solidity-level MSB/log2/bitLength
// Semantic pattern: shift(X) + accumulate(R) + return(R)
// ====================================================

fn detect_solidity_legacy_msb(
    detector: &mut ClzLegacyImplementationsDetector,
    context: &WorkspaceContext,
) {
    use crate::context::browser::{
        ExtractForStatements, ExtractIfStatements, ExtractWhileStatements,
    };

    for function in context.function_definitions() {
        if !function.implemented {
            continue;
        }

        // Already using clz — skip
        if function_uses_clz(context, function.id) {
            continue;
        }

        let Some(body) = &function.body else { continue };

        // Collect shift and accumulate operations
        let mut has_shift = false;
        let mut has_accumulate = false;
        let mut accumulator_names = Vec::<String>::new();

        // Check in if-blocks (Uniswap/PRB style)
        let if_statements = ExtractIfStatements::from(body).extracted;
        let mut power_of_two_count = 0;

        for if_stmt in &if_statements {
            // Check if condition contains power-of-two comparison (2**128, 2**64, etc.)
            if contains_power_of_two_comparison(&if_stmt.condition) {
                power_of_two_count += 1;
            }

            let (shift, acc, names) = analyze_shift_accumulate_in_block_or_stmt(&if_stmt.true_body);
            if shift {
                has_shift = true;
            }
            if acc {
                has_accumulate = true;
                accumulator_names.extend(names);
            }
        }

        // Check in while loops (linear scan style)
        let while_statements = ExtractWhileStatements::from(body).extracted;
        for while_stmt in &while_statements {
            // Check condition for shift pattern: (x >>= 1) > 0
            let (shift_cond, acc_cond, names_cond) =
                analyze_shift_accumulate_expr(&while_stmt.condition);
            if shift_cond {
                has_shift = true;
            }
            if acc_cond {
                has_accumulate = true;
                accumulator_names.extend(names_cond);
            }

            // Check body
            let (shift_body, acc_body, names_body) =
                analyze_shift_accumulate_in_block_or_stmt(&while_stmt.body);
            if shift_body {
                has_shift = true;
            }
            if acc_body {
                has_accumulate = true;
                accumulator_names.extend(names_body);
            }
        }

        // Check in for loops
        let for_statements = ExtractForStatements::from(body).extracted;
        for for_stmt in &for_statements {
            let (shift, acc, names) = analyze_shift_accumulate_in_block_or_stmt(&for_stmt.body);
            if shift {
                has_shift = true;
            }
            if acc {
                has_accumulate = true;
                accumulator_names.extend(names);
            }
        }

        // DECISION LOGIC
        // Strong pattern: Many power-of-two checks (Uniswap style)
        if power_of_two_count >= 3
            && has_shift
            && has_accumulate
            && function_returns_accumulator(context, function.id, &accumulator_names)
        {
            capture!(detector, context, function);
            continue;
        }

        // Semantic Linear Scan Pattern (No name dependency)
        // Check while/for loops for: loop(x > 0) { x >>= 1; count++; }
        if detect_linear_scan_pattern(body) {
            capture!(detector, context, function);
            continue;
        }
    }
}

fn detect_linear_scan_pattern(body: &crate::ast::Block) -> bool {
    use crate::context::browser::{ExtractForStatements, ExtractWhileStatements};

    // 1. Check While Loops
    let while_stmts = ExtractWhileStatements::from(body).extracted;
    for stmt in while_stmts {
        // Condition check: x > 0 or (x >>= 1) > 0
        let (positive_check, condition_has_shift, cond_shifted_var) =
            analyze_condition(&stmt.condition);
        if !positive_check {
            continue;
        }

        // Body check: shift + increment
        let (body_shift, increment, body_shifted_var) = check_body_for_shift_increment(&stmt.body);

        // We need an increment, and a shift EITHER in the condition OR in the body
        if increment && (condition_has_shift || body_shift) {
            // STRICT CHECK 1: Verify x is not modified by other operations
            // Use variable from condition OR body
            let var_to_check = cond_shifted_var.or(body_shifted_var);

            if let Some(var_name) = var_to_check
                && is_variable_modified_by_other_ops(&stmt.body, &var_name)
            {
                continue;
            }
            return true;
        }
    }

    // 2. Check For Loops
    let for_stmts = ExtractForStatements::from(body).extracted;
    for stmt in for_stmts {
        // Condition check
        let mut condition_has_shift = false;
        let mut valid_condition = false;
        let mut cond_shifted_var = None;

        if let Some(cond) = &stmt.condition {
            let (positive_check, cond_shift, var) = analyze_condition(cond);
            if positive_check {
                valid_condition = true;
                condition_has_shift = cond_shift;
                cond_shifted_var = var;
            }
        }

        // If condition is not valid (e.g. i < 256), check for break in body (if (x == 0) break;)
        if !valid_condition && check_body_for_zero_break(&stmt.body) {
            valid_condition = true;
        }

        if !valid_condition {
            continue;
        }

        // Body check
        let (body_shift, body_increment, body_shifted_var) =
            check_body_for_shift_increment(&stmt.body);

        let mut increment = body_increment;

        // Check loop expression for increment (e.g. i++)
        if !increment
            && let Some(loop_expr_stmt) = &stmt.loop_expression
            && check_expression_for_increment(&loop_expr_stmt.expression)
        {
            increment = true;
        }

        if increment && (condition_has_shift || body_shift) {
            // STRICT CHECK 1: Verify x is not modified by other operations
            let var_to_check = cond_shifted_var.or(body_shifted_var);

            if let Some(var_name) = var_to_check
                && is_variable_modified_by_other_ops(&stmt.body, &var_name)
            {
                continue;
            }
            return true;
        }
    }

    false
}

fn check_expression_for_increment(expr: &Expression) -> bool {
    use crate::context::browser::{ExtractAssignments, ExtractUnaryOperations};

    let unary_ops = ExtractUnaryOperations::from(expr).extracted;
    for op in unary_ops {
        if op.operator == "++" {
            return true;
        }
    }

    let assignments = ExtractAssignments::from(expr).extracted;
    for assign in assignments {
        if assign.operator == "+=" {
            return true;
        }
    }

    false
}

fn check_body_for_zero_break(body: &BlockOrStatement) -> bool {
    use crate::context::browser::ExtractIfStatements;

    let if_stmts = ExtractIfStatements::from(body).extracted;
    for stmt in if_stmts {
        // Check condition: x == 0 or iszero(x)
        let mut is_zero_check = false;
        if let Expression::BinaryOperation(bin) = &stmt.condition
            && bin.operator == "=="
        {
            if let Expression::Literal(lit) = bin.right_expression.as_ref() {
                if lit.value.as_deref() == Some("0") {
                    is_zero_check = true;
                }
            } else if let Expression::Literal(lit) = bin.left_expression.as_ref()
                && lit.value.as_deref() == Some("0")
            {
                is_zero_check = true;
            }
        }

        if is_zero_check {
            // Check body for break
            // The body can be a Block or a Statement
            if contains_break(&stmt.true_body) {
                return true;
            }
        }
    }
    false
}

fn contains_break(body: &BlockOrStatement) -> bool {
    match body {
        BlockOrStatement::Block(block) => {
            for stmt in &block.statements {
                if matches!(stmt, crate::ast::Statement::Break(_)) {
                    return true;
                }
            }
        }
        BlockOrStatement::Statement(stmt) => {
            if matches!(stmt.as_ref(), crate::ast::Statement::Break(_)) {
                return true;
            }
        }
    }
    false
}

fn analyze_condition(expr: &Expression) -> (bool, bool, Option<String>) {
    use crate::context::browser::{ExtractAssignments, ExtractBinaryOperations};

    let mut is_positive = false;
    let mut has_shift = false;
    let mut shifted_var = None;

    // Check for > 0 or != 0
    let bin_ops = ExtractBinaryOperations::from(expr).extracted;
    for op in bin_ops {
        if matches!(op.operator.as_str(), ">" | "!=") {
            // Check against 0
            if let Expression::Literal(lit) = op.right_expression.as_ref()
                && lit.value.as_deref() == Some("0")
            {
                is_positive = true;
            }
            if let Expression::Literal(lit) = op.left_expression.as_ref()
                && lit.value.as_deref() == Some("0")
            {
                is_positive = true;
            }
        }
    }

    // Check for embedded shift (x >>= 1)
    let assignments = ExtractAssignments::from(expr).extracted;
    for assign in assignments {
        if matches!(assign.operator.as_str(), ">>=" | ">>") {
            has_shift = true;
            if let Expression::Identifier(id) = assign.left_hand_side.as_ref() {
                shifted_var = Some(id.name.clone());
            }
        }
    }

    (is_positive, has_shift, shifted_var)
}

fn check_body_for_shift_increment(body: &BlockOrStatement) -> (bool, bool, Option<String>) {
    use crate::context::browser::{ExtractAssignments, ExtractUnaryOperations};

    let mut has_shift = false;
    let mut has_increment = false;
    let mut shifted_var = None;

    let assignments = ExtractAssignments::from(body).extracted;
    for assign in assignments {
        match assign.operator.as_str() {
            ">>=" | ">>" => {
                has_shift = true;
                if let Expression::Identifier(id) = assign.left_hand_side.as_ref() {
                    shifted_var = Some(id.name.clone());
                }
            }
            "+=" => {
                // STRICT CHECK 2: Increment only by literal constant
                if let Expression::Literal(_) = assign.right_hand_side.as_ref() {
                    has_increment = true;
                }
            }
            _ => {}
        }
    }

    let unary_ops = ExtractUnaryOperations::from(body).extracted;
    for op in unary_ops {
        if op.operator == "++" {
            has_increment = true;
        }
    }

    (has_shift, has_increment, shifted_var)
}

fn is_variable_modified_by_other_ops(body: &BlockOrStatement, var_name: &str) -> bool {
    use crate::context::browser::ExtractAssignments;
    let assignments = ExtractAssignments::from(body).extracted;
    for assign in assignments {
        // Ignore shift assignments (those are allowed)
        if matches!(assign.operator.as_str(), ">>=" | ">>") {
            continue;
        }

        // Check if LHS is the variable
        if let Expression::Identifier(id) = assign.left_hand_side.as_ref()
            && id.name == var_name
        {
            return true; // Modified by something else!
        }
    }
    false
}
fn analyze_shift_accumulate_in_block_or_stmt(
    block_or_stmt: &BlockOrStatement,
) -> (bool, bool, Vec<String>) {
    use crate::context::browser::{ExtractAssignments, ExtractUnaryOperations};

    let mut has_shift = false;
    let mut has_accumulate = false;
    let mut acc_names = Vec::new();

    // 1. Assignments: x >>= N, r += N, r = r + N
    let assignments = ExtractAssignments::from(block_or_stmt).extracted;
    for assign in assignments {
        match assign.operator.as_str() {
            ">>=" | ">>" => has_shift = true,
            "+=" => {
                has_accumulate = true;
                if let Expression::Identifier(id) = assign.left_hand_side.as_ref() {
                    acc_names.push(id.name.clone());
                }
            }
            "=" => {
                // Check for r = r + N
                if let Expression::Identifier(left_id) = assign.left_hand_side.as_ref() {
                    if let Expression::BinaryOperation(bin_op) = assign.right_hand_side.as_ref()
                        && bin_op.operator == "+"
                    {
                        // Check if left_id is in right side
                        let mut found_in_right = false;
                        if let Expression::Identifier(r_left) = bin_op.left_expression.as_ref()
                            && r_left.name == left_id.name
                        {
                            found_in_right = true;
                        }
                        if let Expression::Identifier(r_right) = bin_op.right_expression.as_ref()
                            && r_right.name == left_id.name
                        {
                            found_in_right = true;
                        }

                        if found_in_right {
                            has_accumulate = true;
                            acc_names.push(left_id.name.clone());
                        }
                    }

                    // Check for r = var (e.g. msb = i)
                    // Avoid r = literal (e.g. y = 5)
                    if !has_accumulate
                        && let Expression::Identifier(_) = assign.right_hand_side.as_ref()
                    {
                        has_accumulate = true;
                        acc_names.push(left_id.name.clone());
                    }
                }
            }
            _ => {}
        }
    }

    // 2. Unary Ops: r++
    let unary_ops = ExtractUnaryOperations::from(block_or_stmt).extracted;
    for op in unary_ops {
        if op.operator == "++" {
            has_accumulate = true;
            if let Expression::Identifier(id) = op.sub_expression.as_ref() {
                acc_names.push(id.name.clone());
            }
        }
    }

    (has_shift, has_accumulate, acc_names)
}

// Analyze expression for shift and accumulate patterns
fn analyze_shift_accumulate_expr(expr: &Expression) -> (bool, bool, Vec<String>) {
    use crate::context::browser::ExtractAssignments;

    let mut has_shift = false;
    let mut has_accumulate = false;
    let mut acc_names = Vec::new();

    // Extract assignments for >>= and += operators
    let assignments = ExtractAssignments::from(expr).extracted;

    for assign in assignments {
        match assign.operator.as_str() {
            ">>=" | ">>" => {
                has_shift = true;
            }
            "+=" => {
                has_accumulate = true;
                if let Expression::Identifier(id) = assign.left_hand_side.as_ref() {
                    acc_names.push(id.name.clone());
                }
            }
            "++" => {
                has_accumulate = true;
                if let Expression::Identifier(id) = assign.left_hand_side.as_ref() {
                    acc_names.push(id.name.clone());
                }
            }
            _ => {}
        }
    }

    (has_shift, has_accumulate, acc_names)
}

// Check if function returns the accumulator
fn function_returns_accumulator(
    context: &WorkspaceContext,
    func_id: NodeID,
    accumulator_names: &[String],
) -> bool {
    use crate::context::browser::ExtractReturns;

    if accumulator_names.is_empty() {
        return false;
    }

    if let Some(crate::ast::ASTNode::FunctionDefinition(func)) = context.nodes.get(&func_id) {
        // Check named return parameters (implicit return)
        for param in &func.return_parameters.parameters {
            if !param.name.is_empty() && accumulator_names.contains(&param.name) {
                return true;
            }
        }

        // Check explicit returns
        if let Some(body) = &func.body {
            let returns = ExtractReturns::from(body).extracted;
            for ret in returns {
                if let Some(expr) = &ret.expression {
                    // return R;
                    if let Expression::Identifier(id) = expr
                        && accumulator_names.contains(&id.name)
                    {
                        return true;
                    }

                    // return R + 1; return 255 - R; etc.
                    if let Expression::BinaryOperation(bin) = expr {
                        if let Expression::Identifier(id_left) = bin.left_expression.as_ref()
                            && accumulator_names.contains(&id_left.name)
                        {
                            return true;
                        }
                        if let Expression::Identifier(id_right) = bin.right_expression.as_ref()
                            && accumulator_names.contains(&id_right.name)
                        {
                            return true;
                        }
                    }
                }
            }
        }
    }

    false
}

// ==========================
// Helper: function uses clz
// ==========================

fn function_uses_clz(context: &WorkspaceContext, func_id: NodeID) -> bool {
    for yul_call in context.yul_function_calls() {
        if yul_call.function_name.name == "clz"
            && let Some(node_context) = context.yul_function_calls_context.get(yul_call)
            && node_context.function_definition_id == Some(func_id)
        {
            return true;
        }
    }
    false
}

#[cfg(test)]
mod clz_legacy_tests {
    use crate::detect::detector::IssueDetector;

    #[test]
    fn test_clz_legacy_detector() {
        let context = crate::detect::test_utils::load_solidity_source_unit(
            "../tests/contract-playground/src/CLZLegacyImplementationsTest.sol",
        );

        let mut detector = super::ClzLegacyImplementationsDetector::default();
        let found = detector.detect(&context).unwrap();
        assert!(found);

        // Get all detected function names by looking up NodeIDs
        let detected_functions: Vec<String> = detector
            .instances()
            .values()
            .filter_map(|node_id| {
                context.nodes.get(node_id).and_then(|node| {
                    if let crate::ast::ASTNode::FunctionDefinition(func) = node {
                        Some(func.name.clone())
                    } else {
                        None
                    }
                })
            })
            .collect();

        println!("Detected functions: {:?}", detected_functions);
        println!("Total detected: {}", detected_functions.len());

        // Expected detections: legacy CLZ/MSB implementations that should be replaced
        // with native CLZ opcode on Osaka+ for better gas efficiency
        let expected = vec![
            "legacyMostSignificantBit", // Binary search pattern (Uniswap/PRBMath style)
            "legacyMostSignificantBitPow2", // Binary search with power-of-2 checks
            "legacyMsb",                // Binary search variant
            "legacyLog2",               // Binary search variant
            "legacyBitLength",          // Binary search variant
            "legacyMostSignificantBitLinear", // Linear scan pattern
            "legacyMostSignificantBitForLoop", // For-loop linear scan
            "legacyBitLengthLinear",    // Linear scan variant
            "legacyMostSignificantBitAsm", // Assembly binary search
            "legacyMostSignificantBitAsmSingle", // Assembly binary search variant
            "legacyClzEIP7939",         // EIP-7939 fallback with magic constants
            "legacyClzWithMagicConstants", // Magic constant pattern
            "scan",                     // Semantic linear scan detection
            "hashValue",                // Semantically identical to linear scan
            "compareLegacyVsModern",    // Wrapper calling legacy implementation
            "testAllLegacyPatterns",    // Wrapper calling legacy implementation
        ];

        for func in &expected {
            assert!(detected_functions.contains(&func.to_string()), "Missing detection: {}", func);
        }

        // Expected FALSE POSITIVES (should NOT be detected)
        let should_not_detect = vec![
            "processBits",
            "packData",
            // "hashValue", // Moved to expected
            "scaleValue",
            "countSomething",
            "extractHighBits",
            "calculatePrice",
            "safeMostSignificantBit",
            "safeBitLength",
            // Regression tests
            "fakeBitLength",
            "mixedFunction",
            "accumulatorFalsePositive",
            "msbCounter",        // New false positive test
            "complexLoop",       // Strict semantic check test
            "variableIncrement", // Strict semantic check test
        ];

        for func in &should_not_detect {
            assert!(!detected_functions.contains(&func.to_string()), "False positive: {}", func);
        }
    }
}
