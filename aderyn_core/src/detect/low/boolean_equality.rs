use crate::{detect::helpers::is_constant_boolean, issue_detector};
use eyre::Result;

issue_detector! {
    BooleanEqualityDetector;

    severity: Low,
    title: "Boolean equality is not required.",
    desc: "If `x` is a boolean, there is no need to do `if(x == true)` or `if(x == false)`. Just use `if(x)` and `if(!x)` respectively.",
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
    use serial_test::serial;

    use crate::detect::{detector::IssueDetector, low::boolean_equality::BooleanEqualityDetector};

    #[test]
    #[serial]
    fn test_boolean_equality_by_loading_contract_directly() {
        let context = crate::detect::test_utils::load_solidity_source_unit(
            "../tests/contract-playground/src/BooleanEquality.sol",
        );

        let mut detector = BooleanEqualityDetector::default();
        let found = detector.detect(&context).unwrap();
        // assert that the detector found an issue
        assert!(found);
        // assert that the detector found the correct number of instances
        assert_eq!(detector.instances().len(), 4);
    }
}
