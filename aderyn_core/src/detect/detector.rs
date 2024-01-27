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
    collections::{BTreeMap, HashMap},
    error::Error,
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

pub fn get_all_detectors_with_ids() -> HashMap<String, Box<dyn Detector>> {
    let mut detectors: HashMap<String, Box<dyn Detector>> = HashMap::new();
    detectors.insert(
        "delegate-call-in-loop".to_string(),
        Box::<DelegateCallInLoopDetector>::default(),
    );
    detectors.insert(
        "centralization-risk".to_string(),
        Box::<CentralizationRiskDetector>::default(),
    );
    detectors.insert(
        "solmate-safe-transfer-lib".to_string(),
        Box::<SolmateSafeTransferLibDetector>::default(),
    );
    detectors.insert(
        "avoid-abi-encode-packed".to_string(),
        Box::<AvoidAbiEncodePackedDetector>::default(),
    );
    detectors.insert("rcrecover".to_string(), Box::<EcrecoverDetector>::default());
    detectors.insert(
        "deprecated-oz-functions".to_string(),
        Box::<DeprecatedOZFunctionsDetector>::default(),
    );
    detectors.insert(
        "unsafe-erc20-functions".to_string(),
        Box::<UnsafeERC20FunctionsDetector>::default(),
    );
    detectors.insert(
        "unspecific-solidity-pragma".to_string(),
        Box::<UnspecificSolidityPragmaDetector>::default(),
    );
    detectors.insert(
        "zero-address-check".to_string(),
        Box::<ZeroAddressCheckDetector>::default(),
    );
    detectors.insert(
        "useless-public-function".to_string(),
        Box::<UselessPublicFunctionDetector>::default(),
    );
    detectors.insert(
        "constants-instead-of-literals".to_string(),
        Box::<ConstantsInsteadOfLiteralsDetector>::default(),
    );
    detectors.insert(
        "unindexed-events".to_string(),
        Box::<UnindexedEventsDetector>::default(),
    );
    detectors.insert(
        "require-with-string".to_string(),
        Box::<RequireWithStringDetector>::default(),
    );
    detectors.insert(
        "non-reentrant-before-others".to_string(),
        Box::<NonReentrantBeforeOthersDetector>::default(),
    );
    detectors.insert(
        "block-timestamp-deadline".to_string(),
        Box::<BlockTimestampDeadlineDetector>::default(),
    );
    detectors.insert(
        "unsafe-erc721-mint".to_string(),
        Box::<UnsafeERC721MintDetector>::default(),
    );
    detectors.insert(
        "push-zero-opcode".to_string(),
        Box::<PushZeroOpcodeDetector>::default(),
    );
    detectors.insert(
        "arbitrary-transfer-from".to_string(),
        Box::<ArbitraryTransferFromDetector>::default(),
    );
    detectors
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
