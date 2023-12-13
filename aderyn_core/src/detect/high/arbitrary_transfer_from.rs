use std::collections::BTreeMap;
use std::error::Error;

use crate::ast::{Expression, FunctionCall, TypeName};

use crate::{
    context::loader::{ASTNode, ContextLoader},
    detect::detector::{Detector, IssueSeverity},
    visitor::ast_visitor::ASTConstVisitor,
};
use eyre::Result;

#[derive(Default)]
pub struct ArbitraryTransferFromDetector {
    // Keys are source file name and line number
    found_instances: BTreeMap<(String, usize), String>,
}

fn check_argument_validity(function_call: &FunctionCall) -> bool {
    let arg_index = if function_call.arguments.len() == 3 {
        0
    } else if function_call.arguments.len() == 4 {
        1
    } else {
        return false;
    };

    match &function_call.arguments[arg_index] {
        Expression::MemberAccess(arg_member_access) => {
            !(arg_member_access.member_name == "sender"
                && matches!(&*arg_member_access.expression, Expression::Identifier(identifier) if identifier.name == "msg"))
        }
        Expression::FunctionCall(arg_function_call) => {
            !(matches!(&*arg_function_call.expression, Expression::ElementaryTypeNameExpression(arg_el_type_name_exp) if matches!(&arg_el_type_name_exp.type_name, TypeName::ElementaryTypeName(type_name) if type_name.name == "address"))
                && matches!(arg_function_call.arguments.get(0), Some(Expression::Identifier(arg_identifier)) if arg_identifier.name == "this"))
        }
        _ => true,
    }
}

impl Detector for ArbitraryTransferFromDetector {
    fn detect(&mut self, loader: &ContextLoader) -> Result<bool, Box<dyn Error>> {
        let transfer_from_function_calls = loader.function_calls.keys().filter(|function_call| {
            if let Expression::MemberAccess(member_access) = &*function_call.expression {
                if member_access.member_name == "transferFrom"
                    || member_access.member_name == "safeTransferFrom"
                {
                    return check_argument_validity(function_call);
                }
            }
            false
        });

        for item in transfer_from_function_calls {
            self.found_instances.insert(
                loader.get_node_sort_key(&ASTNode::FunctionCall(item.clone())),
                item.src.clone(),
            );
        }

        Ok(!self.found_instances.is_empty())
    }

    fn severity(&self) -> IssueSeverity {
        IssueSeverity::High
    }

    fn title(&self) -> String {
        String::from("")
    }

    fn description(&self) -> String {
        String::from("")
    }

    fn instances(&self) -> BTreeMap<(String, usize), String> {
        self.found_instances.clone()
    }
}

#[cfg(test)]
mod arbitrary_transfer_from_tests {
    use crate::detect::{
        detector::{detector_test_helpers::load_contract, Detector},
        high::arbitrary_transfer_from::ArbitraryTransferFromDetector,
    };

    #[test]
    fn test_arbitrary_transfer_from_detector() {
        let context_loader = load_contract(
            "../tests/contract-playground/out/ArbitraryTransferFrom.sol/ArbitraryTransferFrom.json",
        );
        let mut detector = ArbitraryTransferFromDetector::default();
        let found = detector.detect(&context_loader).unwrap();
        // assert that the detector found an issue
        assert!(found);
        // assert that the detector found the correct number of instances
        assert_eq!(detector.instances().len(), 4);
        // assert the severity is high
        assert_eq!(
            detector.severity(),
            crate::detect::detector::IssueSeverity::High
        );
        // assert the title is correct
        assert_eq!(detector.title(), String::from(""));
        // assert the description is correct
        assert_eq!(detector.description(), String::from(""));
    }
}
