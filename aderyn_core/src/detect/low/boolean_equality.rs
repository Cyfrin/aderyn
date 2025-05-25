use crate::{detect::helpers::is_constant_boolean, issue_detector};
use eyre::Result;

issue_detector! {
    BooleanEqualityDetector;

    severity: Low,
    title: "Boolean equality is not required",
    desc: "If `x` is a boolean, use `if(x)` and `if(!x)` instead of `if(x == true)` or `if(x == false)`.",
    name: BooleanEquality,

    |context| {
        for binary_operation in context.binary_operations() {
            if binary_operation.operator == "=="
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
    }

}

#[cfg(test)]
mod boolean_equality_tests {

    use crate::detect::{detector::IssueDetector, low::boolean_equality::BooleanEqualityDetector};

    #[test]

    fn test_boolean_equality_by_loading_contract_directly() {
        let context = crate::detect::test_utils::load_solidity_source_unit(
            "../tests/contract-playground/src/BooleanEquality.sol",
        );

        let mut detector = BooleanEqualityDetector::default();
        let found = detector.detect(&context).unwrap();
        assert!(found);
        assert_eq!(detector.instances().len(), 4);
    }
}
