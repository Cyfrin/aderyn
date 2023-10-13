use crate::{
    detector::{
        high::delegate_call_in_loop::DelegateCallInLoopDetector,
        medium::centralization_risk::CentralizationRiskDetector,
    },
    loader::loader::{ASTNode, ContractLoader},
};
use std::error::Error;

pub fn get_all_detectors() -> Vec<Box<dyn Detector>> {
    vec![
        Box::new(DelegateCallInLoopDetector::default()),
        Box::new(CentralizationRiskDetector::default()),
    ]
}

#[derive(Debug, PartialEq)]
pub enum IssueSeverity {
    Gas,
    NC,
    Low,
    Medium,
    High,
    Critical,
}

pub trait Detector {
    fn detect(&mut self, _loader: &ContractLoader) -> Result<bool, Box<dyn Error>> {
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
        loader::loader::ContractLoader, read_foundry_output_file, visitor::ast_visitor::Node,
    };

    pub fn load_contract(filepath: &str) -> ContractLoader {
        let filepath = std::path::PathBuf::from(filepath);
        let mut contract_loader = ContractLoader::default();
        let foundry_output = read_foundry_output_file(filepath.to_str().unwrap()).unwrap();
        let _ = foundry_output.ast.accept(&mut contract_loader);
        contract_loader
    }
}
