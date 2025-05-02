use crate::{detect::helpers::is_constant_boolean, issue_detector};
use eyre::Result;

issue_detector! {
    MisusedBooleanDetector;

    severity: High,
    title: "Misused boolean with logical operators",
    desc: "The patterns `if (… || true)` and `if (.. && false)` will always evaluate to true and false respectively.",
    name: MisusedBoolean,

    |context| {
        for binary_operation in context.binary_operations() {
            if (binary_operation.operator == "||" || binary_operation.operator == "&&")
                && [
                    binary_operation.left_expression.as_ref(),
                    binary_operation.right_expression.as_ref(),
                ]
                .iter()
                .any(|&operand| is_constant_boolean(context, operand))
            {
                grab!(binary_operation);
            }

        }

        for if_statement in context.if_statements()
            .iter()
            .filter(|statement| is_constant_boolean(context, &statement.condition)) {
            grab!(if_statement);
        }

    }

}

#[cfg(test)]
mod misused_boolean_tests {

    use crate::detect::{detector::IssueDetector, high::misused_boolean::MisusedBooleanDetector};

    #[test]

    fn test_misused_boolean_by_loading_contract_directly() {
        let context = crate::detect::test_utils::load_solidity_source_unit(
            "../tests/contract-playground/src/MisusedBoolean.sol",
        );

        let mut detector = MisusedBooleanDetector::default();
        let found = detector.detect(&context).unwrap();
        assert!(found);
        assert_eq!(detector.instances().len(), 10);
    }
}
