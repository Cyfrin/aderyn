use crate::loader::loader::{ASTNode, ContractLoader};
use std::error::Error;

pub fn get_all_detectors() -> Vec<Box<dyn Detector>> {
    vec![Box::new(
        crate::detector::high::delegate_call_in_loop::DelegateCallInLoopDetector::default(),
    )]
}

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
