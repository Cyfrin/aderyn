use crate::{
    context::loader::ContextLoader,
    detect::{
        high::{
            arbitrary_transfer_from::ArbitraryTransferFromDetector,
            delegate_call_in_loop::DelegateCallInLoopDetector,
        },
        low::{
            avoid_abi_encode_packed::AvoidAbiEncodePackedDetector,
            deprecated_oz_functions::DeprecatedOZFunctionsDetector, ecrecover::EcrecoverDetector,
            push_0_opcode::PushZeroOpcodeDetector,
            unsafe_erc20_functions::UnsafeERC20FunctionsDetector,
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
use std::{
    collections::BTreeMap,
    error::Error,
    fmt::{self, Display},
};

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
        Box::<PushZeroOpcodeDetector>::default(),
        Box::<ArbitraryTransferFromDetector>::default(),
    ]
}

pub fn get_all_detectors_ids() -> Vec<String> {
    vec![
        "delegate-call-in-loop".to_string(),
        "centralization-risk".to_string(),
        "solmate-safe-transfer-lib".to_string(),
        "avoid-abi-encode-packed".to_string(),
        "ercrecover".to_string(),
        "deprecated-oz-functions".to_string(),
        "unsafe-erc20-functions".to_string(),
        "unspecific-solidity-pragma".to_string(),
        "zero-address-check".to_string(),
        "useless-public-function".to_string(),
        "constants-instead-of-literals".to_string(),
        "unindexed-events".to_string(),
        "require-with-string".to_string(),
        "non-reentrant-before-others".to_string(),
        "block-timestamp-deadline".to_string(),
        "unsafe-erc721-mint".to_string(),
        "push-zero-opcode".to_string(),
        "arbitrary-transfer-from".to_string(),
    ]
}

pub fn get_detector_by_id(detector_id: &str) -> Box<dyn Detector> {
    // Expects a valid detector_id
    match detector_id {
        "delegate-call-in-loop" => Box::<DelegateCallInLoopDetector>::default(),
        "centralization-risk" => Box::<CentralizationRiskDetector>::default(),
        "solmate-safe-transfer-lib" => Box::<SolmateSafeTransferLibDetector>::default(),
        "avoid-abi-encode-packed" => Box::<AvoidAbiEncodePackedDetector>::default(),
        "ercrecover" => Box::<EcrecoverDetector>::default(),
        "deprecated-oz-functions" => Box::<DeprecatedOZFunctionsDetector>::default(),
        "unsafe-erc20-functions" => Box::<UnsafeERC20FunctionsDetector>::default(),
        "unspecific-solidity-pragma" => Box::<UnspecificSolidityPragmaDetector>::default(),
        "zero-address-check" => Box::<ZeroAddressCheckDetector>::default(),
        "useless-public-function" => Box::<UselessPublicFunctionDetector>::default(),
        "constants-instead-of-literals" => Box::<ConstantsInsteadOfLiteralsDetector>::default(),
        "unindexed-events" => Box::<UnindexedEventsDetector>::default(),
        "require-with-string" => Box::<RequireWithStringDetector>::default(),
        "non-reentrant-before-others" => Box::<NonReentrantBeforeOthersDetector>::default(),
        "block-timestamp-deadline" => Box::<BlockTimestampDeadlineDetector>::default(),
        "unsafe-erc721-mint" => Box::<UnsafeERC20FunctionsDetector>::default(),
        "push-zero-opcode" => Box::<PushZeroOpcodeDetector>::default(),
        "arbitrary-transfer-from" => Box::<ArbitraryTransferFromDetector>::default(),
        _ => panic!("Invalid detector ID!"),
    }
}

#[derive(Debug, PartialEq)]
pub enum IssueSeverity {
    NC,
    Low,
    Medium,
    High,
    Critical,
}

impl Display for IssueSeverity {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let issue_description = match self {
            IssueSeverity::NC => "NC (Non Critical)",
            IssueSeverity::Low => "Low",
            IssueSeverity::Medium => "Medium",
            IssueSeverity::High => "High",
            IssueSeverity::Critical => "Critical",
        };
        write!(f, "{}", issue_description).unwrap();
        Ok(())
    }
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
        let mut ast = foundry_output.ast.clone();
        // Get the path of the source file
        let mut new_path = PathBuf::new();
        for component in path_buf_filepath.components() {
            if component.as_os_str() == "out" {
                break;
            }
            new_path.push(component);
        }
        new_path.push(ast.absolute_path.as_ref().unwrap());
        match read_file_to_string(&new_path) {
            Ok(content) => {
                println!(
                    "Loaded Solidity source file: {}",
                    new_path.to_str().unwrap()
                );

                ast.source = Some(content);
            }
            Err(err) => {
                eprintln!(
                    "Error reading Solidity source file: {}",
                    new_path.to_str().unwrap()
                );
                eprintln!("{:?}", err);
            }
        }
        ast.accept(&mut context_loader).unwrap_or_else(|err| {
            // Exit with a non-zero exit code
            eprintln!("Error loading Hardhat AST into ContextLoader");
            eprintln!("{:?}", err);
        });
        context_loader
    }
}
