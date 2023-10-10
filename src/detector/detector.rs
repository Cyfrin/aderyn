use crate::loader::loader::ContractLoader;
use std::error::Error;

pub trait Detector {
    fn detect(&mut self, loader: &ContractLoader) -> Result<(), Box<dyn Error>>{
        Ok(())
    }
}