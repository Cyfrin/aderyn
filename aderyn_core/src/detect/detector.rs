use crate::{
    context::loader::ContextLoader,
    detect::{
        high::delegate_call_in_loop::DelegateCallInLoopDetector,
        low::{
            avoid_abi_encode_packed::AvoidAbiEncodePackedDetector,
            deprecated_oz_functions::DeprecatedOZFunctionsDetector,
            different_storage_conditionals::DifferentStorageConditionalDetector,
            ecrecover::EcrecoverDetector, unsafe_erc20_functions::UnsafeERC20FunctionsDetector,
            unspecific_solidity_pragma::UnspecificSolidityPragmaDetector,
        },
        medium::{
            block_timestamp_deadline::BlockTimestampDeadlineDetector,
            centralization_risk::CentralizationRiskDetector,
            solmate_safe_transfer_lib::SolmateSafeTransferLibDetector,
            unsafe_oz_erc721_mint::UnsafeERC721MintDetector,
        },
        nc::{
            constants_instead_of_literals::ConstantsInsteadOfLiteralsDetector,
            non_reentrant_before_others::NonReentrantBeforeOthersDetector,
            require_with_string::RequireWithStringDetector,
            unindexed_events::UnindexedEventsDetector,
            useless_public_function::UselessPublicFunctionDetector,
            zero_address_check::ZeroAddressCheckDetector,
        },
    },
};
use std::{collections::BTreeMap, error::Error};

pub fn get_all_detectors() -> Vec<Box<dyn Detector>> {
    vec![
        Box::<DelegateCallInLoopDetector>::default(),
        Box::<CentralizationRiskDetector>::default(),
        Box::<SolmateSafeTransferLibDetector>::default(),
        Box::<AvoidAbiEncodePackedDetector>::default(),
        Box::<EcrecoverDetector>::default(),
        Box::<DeprecatedOZFunctionsDetector>::default(),
        Box::<UnsafeERC20FunctionsDetector>::default(),
        Box::<UnspecificSolidityPragmaDetector>::default(),
        Box::<ZeroAddressCheckDetector>::default(),
        Box::<UselessPublicFunctionDetector>::default(),
        Box::<ConstantsInsteadOfLiteralsDetector>::default(),
        Box::<UnindexedEventsDetector>::default(),
        Box::<RequireWithStringDetector>::default(),
        Box::<NonReentrantBeforeOthersDetector>::default(),
        Box::<BlockTimestampDeadlineDetector>::default(),
        Box::<UnsafeERC721MintDetector>::default(),
        Box::<DifferentStorageConditionalDetector>::default(),
    ]
}

#[derive(Debug, PartialEq)]
pub enum IssueSeverity {
    NC,
    Low,
    Medium,
    High,
    Critical,
}

pub trait Detector {
    fn detect(&mut self, _loader: &ContextLoader) -> Result<bool, Box<dyn Error>> {
        Ok(true)
    }

    fn severity(&self) -> IssueSeverity {
        IssueSeverity::Medium
    }

    fn title(&self) -> String {
        String::from("Title")
    }

    fn description(&self) -> String {
        String::from("Description")
    }

    // Keys are source file name and line number
    // Value is ASTNode.src
    fn instances(&self) -> BTreeMap<(String, usize), String> {
        BTreeMap::new()
    }
}

pub mod detector_test_helpers {
    use std::path::PathBuf;

    use crate::{
        context::loader::ContextLoader, framework::foundry::read_foundry_output_file,
        read_file_to_string, visitor::ast_visitor::Node,
    };

    pub fn load_contract(filepath: &str) -> ContextLoader {
        let path_buf_filepath = std::path::PathBuf::from(filepath);
        let mut context_loader = ContextLoader::default();
        let foundry_output = read_foundry_output_file(path_buf_filepath.to_str().unwrap()).unwrap();
        let _ = foundry_output.ast.accept(&mut context_loader);
        // Get the path of the source file
        let mut new_path = PathBuf::new();
        for component in path_buf_filepath.components() {
            if component.as_os_str() == "out" {
                break;
            }
            new_path.push(component);
        }
        new_path.push(foundry_output.ast.absolute_path.unwrap());
        match read_file_to_string(&new_path) {
            Ok(content) => {
                println!(
                    "Loaded Solidity source file: {}",
                    new_path.to_str().unwrap()
                );
                // Convert the full_path to a string
                let full_path_str = new_path.to_str().unwrap_or("");

                // Find the index where "src/" starts
                if let Some(start_index) = full_path_str.find("src/") {
                    let target_path = &full_path_str[start_index..];

                    // Search for a match and modify
                    for unit in context_loader.source_units.iter() {
                        if let Some(ref abs_path) = unit.absolute_path {
                            if abs_path == target_path {
                                context_loader.set_source_unit_source_content(unit.id, content);
                                break;
                            }
                        }
                    }
                }
            }
            Err(err) => {
                eprintln!(
                    "Error reading Solidity source file: {}",
                    new_path.to_str().unwrap()
                );
                eprintln!("{:?}", err);
            }
        }
        context_loader
    }
}
