use crate::loader::loader::ContractLoader;
use std::error::Error;

pub enum IssueSeverity {
    Gas,
    Low,
    Medium,
    High,
    Critical,
}

pub trait Detector<T> {
    fn detect(&mut self, _loader: &ContractLoader) -> Result<(), Box<dyn Error>>{
        Ok(())
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

    fn get_instances(&self) -> Vec<T> {
        Vec::new()
    }
}