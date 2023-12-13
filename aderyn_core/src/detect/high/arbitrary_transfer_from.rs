use std::collections::{BTreeMap, HashMap};
use std::error::Error;

use crate::ast::{ElementaryTypeName, Expression, FunctionCall, FunctionDefinition, TypeName};
use crate::visitor::ast_visitor::Node;
use crate::{
    ast::MemberAccess,
    context::loader::{ASTNode, ContextLoader},
    detect::detector::{Detector, IssueSeverity},
    visitor::ast_visitor::ASTConstVisitor,
};
use eyre::Result;
use rayon::iter;

#[derive(Default)]
pub struct ArbitraryTransferFromDetector {
    // Keys are source file name and line number
    found_instances: BTreeMap<(String, usize), String>,
}

impl ASTConstVisitor for ArbitraryTransferFromDetector {}

impl Detector for ArbitraryTransferFromDetector {
    fn detect(&mut self, loader: &ContextLoader) -> Result<bool, Box<dyn Error>> {
        // Get all FunctionCalls where the expression is a MemberAccess with the member name "transferFrom",
        // Add them to a list of FunctionCalls to check
        let transfer_from_function_calls = loader.function_calls.keys().filter(|function_call| {
            if let Expression::MemberAccess(member_access) = &*function_call.expression {
                if member_access.member_name == "transferFrom"
                    || member_access.member_name == "safeTransferFrom"
                {
                    if function_call.arguments.len() == 3 {
                        // if the first argument is NOT a MemberAccess with member name "sender"
                        // and an Identifier with the name "msg",
                        // return true
                        match &function_call.arguments[0] {
                            Expression::MemberAccess(arg_member_access) => {
                                if arg_member_access.member_name == "sender" {
                                    if let Expression::Identifier(arg_member_access_expression) =
                                        &*arg_member_access.expression
                                    {
                                        if arg_member_access_expression.name == "msg" {
                                            return false;
                                        }
                                    }
                                }
                            }
                            Expression::FunctionCall(arg_function_call) => {
                                if let Expression::ElementaryTypeNameExpression(
                                    arg_el_type_name_exp,
                                ) = &*arg_function_call.expression
                                {
                                    if let TypeName::ElementaryTypeName(type_name) =
                                        &arg_el_type_name_exp.type_name
                                    {
                                        if type_name.name == "address" {
                                            if let Expression::Identifier(arg_identifier) =
                                                &arg_function_call.arguments[0]
                                            {
                                                if arg_identifier.name == "this" {
                                                    return false;
                                                }
                                            }
                                        }
                                    }
                                }
                            }
                            _ => {
                                return true;
                            }
                        }
                    } else if function_call.arguments.len() == 4 {
                        // if the second argument is NOT a MemberAccess with member name "sender"
                        // and an Identifier with the name "msg",
                        // return true
                        match &function_call.arguments[1] {
                            Expression::MemberAccess(arg_member_access) => {
                                if arg_member_access.member_name == "sender" {
                                    if let Expression::Identifier(arg_member_access_expression) =
                                        &*arg_member_access.expression
                                    {
                                        if arg_member_access_expression.name == "msg" {
                                            return false;
                                        }
                                    }
                                }
                            }
                            Expression::FunctionCall(arg_function_call) => {
                                if let Expression::ElementaryTypeNameExpression(
                                    arg_el_type_name_exp,
                                ) = &*arg_function_call.expression
                                {
                                    if let TypeName::ElementaryTypeName(type_name) =
                                        &arg_el_type_name_exp.type_name
                                    {
                                        if type_name.name == "address" {
                                            if let Expression::Identifier(arg_identifier) =
                                                &arg_function_call.arguments[0]
                                            {
                                                if arg_identifier.name == "this" {
                                                    return false;
                                                }
                                            }
                                        }
                                    }
                                }
                            }
                            _ => {
                                return true;
                            }
                        }
                    }
                }
            }
            return false;
        });

        for item in transfer_from_function_calls.collect::<Vec<_>>() {
            self.found_instances.insert(
                loader.get_node_sort_key(&ASTNode::FunctionCall(item.clone())),
                item.src.clone(),
            );
        }

        // let sources = &transfer_from_function_calls
        //     .into_iter()
        //     .map(|call| &call.src)
        //     .collect::<Vec<&String>>();
        // let json = serde_json::to_string(sources).unwrap();
        // println!("transfer_from_function_calls: {}", &json);

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
