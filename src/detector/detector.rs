use crate::{
    context::loader::{ASTNode, ContextLoader},
    detector::{
        high::delegate_call_in_loop::DelegateCallInLoopDetector,
        low::{
            avoid_abi_encode_packed::AvoidAbiEncodePackedDetector,
            deprecated_oz_functions::DeprecatedOZFunctionsDetector, ecrecover::EcrecoverDetector,
            unsafe_erc20_functions::UnsafeERC20FunctionsDetector,
            unspecific_solidity_pragma::UnspecificSolidityPragmaDetector,
        },
        medium::{
            centralization_risk::CentralizationRiskDetector,
            solmate_safe_transfer_lib::SolmateSafeTransferLibDetector,
        },
        nc::{
            constants_instead_of_literals::ConstantsInsteadOfLiteralsDetector,
            require_with_string::RequireWithStringDetector,
            unindexed_events::UnindexedEventsDetector,
            useless_public_function::UselessPublicFunctionDetector,
            zero_address_check::ZeroAddressCheckDetector,
        },
    },
};
use std::error::Error;

pub fn get_all_detectors() -> Vec<Box<dyn Detector>> {
    vec![
        Box::new(DelegateCallInLoopDetector::default()),
        Box::new(CentralizationRiskDetector::default()),
        Box::new(SolmateSafeTransferLibDetector::default()),
        Box::new(AvoidAbiEncodePackedDetector::default()),
        Box::new(EcrecoverDetector::default()),
        Box::new(DeprecatedOZFunctionsDetector::default()),
        Box::new(UnsafeERC20FunctionsDetector::default()),
        Box::new(UnspecificSolidityPragmaDetector::default()),
        Box::new(ZeroAddressCheckDetector::default()),
        Box::new(UselessPublicFunctionDetector::default()),
        Box::new(ConstantsInsteadOfLiteralsDetector::default()),
        Box::new(UnindexedEventsDetector::default()),
        Box::new(RequireWithStringDetector::default()),
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

    fn instances(&self) -> Vec<Option<ASTNode>> {
        Vec::new()
    }
}

pub mod detector_test_helpers {
    use crate::{
        context::loader::ContextLoader, read_foundry_output_file, visitor::ast_visitor::Node,
    };

    pub fn load_contract(filepath: &str) -> ContextLoader {
        let filepath = std::path::PathBuf::from(filepath);
        let mut context_loader = ContextLoader::default();
        let foundry_output = read_foundry_output_file(filepath.to_str().unwrap()).unwrap();
        let _ = foundry_output.ast.accept(&mut context_loader);
        context_loader
    }
}
